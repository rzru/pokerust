use crate::abilities::renderable_ability::RenderableAbility;
use crate::abilities::{PokemonAbility, PokemonAbilityExt};
use crate::fetch_external;
use crate::held_item::PokemonHeldItem;
use crate::http::Http;
use crate::moves::{PokemonMove, PokemonMoveExt, RenderableMove};
use crate::named_api_resource::NamedApiResource;
use crate::pk_type::{PokemonType, Type};
use crate::pokemon_species::PokemonSpecies;
use crate::pokemon_stat::PokemonStat;
use crate::sprites::PokemonSprites;
use crate::version_game_index::VersionGameIndex;
use crate::BASE_URL;
use owo_colors::OwoColorize;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use tabled::{Alignment, Column, Disable, Format, Full, MaxWidth, Modify, Row, Table};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub base_experience: Option<i32>,
    pub height: Option<i32>,
    pub is_default: Option<bool>,
    pub order: Option<i32>,
    pub weight: Option<i32>,
    pub abilities: Option<Vec<PokemonAbility>>,
    pub forms: Option<Vec<NamedApiResource>>,
    pub game_indices: Option<Vec<VersionGameIndex>>,
    pub held_items: Option<Vec<PokemonHeldItem>>,
    pub location_area_encounters: Option<String>,
    pub moves: Option<Vec<PokemonMove>>,
    pub sprites: Option<PokemonSprites>,
    pub species: Option<NamedApiResource>,
    pub stats: Option<Vec<PokemonStat>>,
    pub types: Option<Vec<PokemonType>>,
}

impl Pokemon {
    pub async fn new(client: &Http, name_or_id: &str) -> Option<Self> {
        let uri = format!("{}/pokemon/{}", BASE_URL, name_or_id);

        let data = client.get(&uri).await;

        if let Some(bytes) = data {
            let pokemon: Self = serde_json::from_slice(&bytes).unwrap();

            return Some(pokemon);
        }

        None
    }

    fn prepare_abilities(&self) -> Vec<String> {
        self.abilities
            .as_ref()
            .unwrap()
            .iter()
            .map(|ability| {
                ability
                    .ability
                    .as_ref()
                    .unwrap()
                    .name
                    .as_ref()
                    .unwrap()
                    .to_string()
            })
            .collect()
    }

    fn prepare_types(&self) -> Vec<String> {
        self.types
            .as_ref()
            .unwrap()
            .iter()
            .map(|pk_type| {
                let name = pk_type.de_type.as_ref().unwrap().name.as_ref().unwrap();
                Type::get(name, name).color_fmt()
            })
            .collect()
    }

    fn prepare_games(&self) -> Vec<String> {
        self.game_indices
            .as_ref()
            .unwrap()
            .iter()
            .map(|gi| {
                let name = gi.version.as_ref().unwrap().name.as_ref().unwrap();

                name.clone()
            })
            .collect()
    }

    fn prepare_stats(&self) -> Vec<String> {
        self.stats
            .as_ref()
            .unwrap()
            .iter()
            .map(|stat| stat.to_string())
            .collect()
    }

    pub async fn render(&self, render_details: HashMap<&str, bool>, gg: String) -> io::Result<()> {
        let mut base_info_data = vec![
            ("ID", self.id.unwrap().to_string()),
            ("Name", self.name.as_ref().unwrap().to_string()),
            ("Base Experience", self.base_experience.unwrap().to_string()),
            ("Height", self.height.unwrap().to_string()),
            ("Weight", self.weight.unwrap().to_string()),
            ("Types", self.prepare_types().join(", ")),
            ("Base stats", self.prepare_stats().join(", ")),
            ("Appears at", self.prepare_games().join(", ")),
        ];

        if *render_details.get("ext").unwrap() {
            fn fetch_species_url(pokemon: &&Pokemon) -> String {
                pokemon
                    .species
                    .as_ref()
                    .unwrap()
                    .url
                    .as_ref()
                    .unwrap()
                    .to_string()
            }

            let pokemon_species_info_vec = fetch_external::<&Pokemon, PokemonSpecies>(
                vec![self].as_slice(),
                fetch_species_url,
            )
            .await;
            let pokemon_species_info = pokemon_species_info_vec.first();

            if let Some(pokemon_species_info) = pokemon_species_info {
                let mut species_info_entries = vec![
                    ("Base Happiness", pokemon_species_info.base_happiness()),
                    ("Capture Rate", pokemon_species_info.capture_rate()),
                    ("Gender Rate", pokemon_species_info.gender_rate()),
                    ("Is legendary", pokemon_species_info.is_legendary()),
                    ("Description", pokemon_species_info.description()),
                ];

                base_info_data.append(&mut species_info_entries)
            }
        }

        base_info_data.push((
            "Abilities",
            if *render_details.get("abilities").unwrap() {
                self.prepare_abilities_table(&gg).await.unwrap()
            } else {
                self.prepare_abilities().join(", ")
            },
        ));

        if *render_details.get("moves").unwrap() {
            base_info_data.push(("Moves", self.prepare_moves_table(&gg).await.unwrap()));
        }

        let base_info = Table::new(&base_info_data)
            .with(Disable::Row(..1))
            .with(Modify::new(Column(..1)).with(Format(|s| s.bold().to_string())))
            .with(Modify::new(Row(7..8)).with(MaxWidth::wrapping(80)))
            .with(
                Modify::new(Full)
                    .with(Alignment::left())
                    .with(Alignment::top()),
            )
            .to_string();

        println!("{}", base_info);

        Ok(())
    }

    async fn prepare_abilities_table(&self, gg: &str) -> Option<String> {
        if let Some(abilities) = self.abilities.as_ref() {
            fn fetch_move_url(ability: &PokemonAbility) -> String {
                String::from(ability.ability.as_ref().unwrap().url.as_ref().unwrap())
            }

            let externals =
                fetch_external::<PokemonAbility, PokemonAbilityExt>(abilities, fetch_move_url)
                    .await;

            let prepared_abilities: Vec<RenderableAbility> = abilities
                .par_iter()
                .map(|ability| {
                    let ext = externals.iter().find(|item| {
                        item.name.as_ref().unwrap()
                            == ability.ability.as_ref().unwrap().name.as_ref().unwrap()
                    });
                    ability.to_renderable(gg, ext)
                })
                .flatten()
                .collect();

            let table = Table::new(&prepared_abilities)
                .with(
                    Modify::new(Full)
                        .with(Alignment::left())
                        .with(Alignment::top()),
                )
                .with(Modify::new(Column(..1)).with(Format(|s| s.bold().to_string())))
                .with(Modify::new(Column(1..2)).with(MaxWidth::wrapping(20)))
                .with(Modify::new(Column(2..3)).with(MaxWidth::wrapping(35)))
                .to_string();

            return Some(table);
        }

        None
    }

    async fn prepare_moves_table(&self, gg: &str) -> Option<String> {
        if let Some(moves) = self.moves.as_ref() {
            fn fetch_move_url(mv: &PokemonMove) -> String {
                String::from(mv.de_move.as_ref().unwrap().url.as_ref().unwrap())
            }

            let externals =
                fetch_external::<PokemonMove, PokemonMoveExt>(moves, fetch_move_url).await;

            let mut prepared_moves: Vec<RenderableMove> = moves
                .par_iter()
                .map(|mv| {
                    let ext = externals.iter().find(|item| {
                        item.name.as_ref().unwrap()
                            == mv.de_move.as_ref().unwrap().name.as_ref().unwrap()
                    });
                    mv.to_renderable(gg, ext)
                })
                .flatten()
                .collect();

            prepared_moves.sort_by_key(|mv| (mv.move_learn_method, mv.level_learned_at));

            let table = Table::new(&prepared_moves)
                .with(
                    Modify::new(Full)
                        .with(Alignment::left())
                        .with(Alignment::top()),
                )
                .with(Modify::new(Column(..1)).with(Format(|s| s.bold().to_string())))
                .to_string();

            return Some(table);
        }

        None
    }
}
