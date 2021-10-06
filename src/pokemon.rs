use crate::ability::PokemonAbility;
use crate::held_item::PokemonHeldItem;
use crate::http::Http;
use crate::named_api_resource::NamedApiResource;
use crate::pk_move::{PokemonMove, PokemonMoveExt, RenderableMove};
use crate::pk_type::{PokemonType, Type};
use crate::sprites::PokemonSprites;
use crate::version_game_index::VersionGameIndex;
use crate::BASE_URL;
use owo_colors::OwoColorize;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::io;
use std::sync::{Arc, Mutex};
use tabled::{Alignment, Column, Disable, Format, Full, MaxWidth, Modify, Row, Style, Table};

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

    pub async fn render(&self, should_render_moves: bool, gg: Option<&str>) -> io::Result<()> {
        let mut base_info_data = vec![
            ("ID", self.id.unwrap().to_string()),
            ("Name", self.name.as_ref().unwrap().to_string()),
            ("Base Experience", self.base_experience.unwrap().to_string()),
            ("Height", self.height.unwrap().to_string()),
            ("Weight", self.weight.unwrap().to_string()),
        ];

        base_info_data.push(("Types", self.prepare_types().join(", ")));
        base_info_data.push(("Appears at", self.prepare_games().join(", ")));

        if should_render_moves {
            let info = self.prepare_moves_table(gg.unwrap()).await.unwrap();
            base_info_data.push(("Moves", info));
        }

        let base_info = Table::new(&base_info_data)
            .with(Disable::Row(..1))
            .with(Modify::new(Column(..1)).with(Format(|s| s.green().bold().to_string())))
            .with(Modify::new(Row(6..7)).with(MaxWidth::wrapping(85)))
            .with(
                Modify::new(Full)
                    .with(Alignment::left())
                    .with(Alignment::center_vertical()),
            )
            .to_string();

        println!("{}", base_info);

        Ok(())
    }

    async fn prepare_moves_table(&self, gg: &str) -> Option<String> {
        if let Some(moves) = self.moves.as_ref() {
            let mut handles = vec![];
            let pk_moves_ext = Arc::new(Mutex::new(vec![]));

            for mv in moves {
                let url = String::from(mv.de_move.as_ref().unwrap().url.as_ref().unwrap());
                let pk_moves_ext = pk_moves_ext.clone();
                let handle = tokio::spawn(async move {
                    let http = Http::new();
                    let data = http.get(&url).await;

                    if let Some(bytes) = data {
                        let mut pk_moves_ext = pk_moves_ext.lock().unwrap();
                        let pk_move_ext: PokemonMoveExt = serde_json::from_slice(&bytes).unwrap();

                        pk_moves_ext.push(pk_move_ext);
                    }
                });

                handles.push(handle)
            }

            for handle in handles {
                handle.await.unwrap();
            }

            let mut prepared_moves: Vec<RenderableMove> = moves
                .par_iter()
                .map(|mv| {
                    let externals = &*pk_moves_ext.lock().unwrap();
                    let ext = externals.iter().find(|item| {
                        item.name.as_ref().unwrap() == mv.de_move.as_ref().unwrap().name.as_ref().unwrap()
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
                        .with(Alignment::center_vertical()),
                )
                .with(Style::default())
                .to_string();

            return Some(table);
        }

        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonStat {
    pub stat: Option<NamedApiResource>,
    pub effort: Option<i32>,
    pub base_stat: Option<i32>,
}
