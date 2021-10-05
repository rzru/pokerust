use serde::{Serialize, Deserialize};
use crate::http::Http;
use crate::{BASE_URL};
use std::io;
use tabled::{Tabled, Table, Modify, Full, Alignment, Disable};

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

    fn render_moves(&self, gg: &str) -> Option<String> {
        if let Some(moves) = self.moves.as_ref() {
            let mut prepared_moves: Vec<RenderableMove> = moves
                .iter()
                .map(|mv| mv.to_renderable(gg))
                .flatten()
                .collect();

            prepared_moves.sort_by_key(|mv| (mv.move_learn_method, mv.level_learned_at));

            let table = Table::new(&prepared_moves)
                .with(Modify::new(Full)
                    .with(Alignment::left())
                    .with(Alignment::center_vertical())
                )
                .to_string();

            return Some(table);
        }

        None
    }

    pub fn render(&self, should_render_moves: bool, gg: Option<&str>) -> io::Result<()> {
        let mut base_info_data = vec![
            ("ID", self.id.unwrap().to_string()),
            ("Name", self.name.as_ref().unwrap().to_string()),
            ("Base Experience", self.base_experience.unwrap().to_string()),
            ("Height", self.height.unwrap().to_string()),
            ("Weight", self.weight.unwrap().to_string()),
        ];

        if should_render_moves {
            let info = self.render_moves(gg.unwrap()).unwrap();
            base_info_data.push(("Moves", info));
        }

        let base_info = Table::new(base_info_data)
            .with(Disable::Row(..1))
            .with(Modify::new(Full)
                .with(Alignment::left())
                .with(Alignment::center_vertical())
            )
            .to_string();

        println!("{}", base_info);



        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonAbility {
    pub is_hidden: Option<bool>,
    pub slot: Option<i32>,
    pub abilities: Option<Vec<NamedApiResource>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NamedApiResource {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionGameIndex {
    pub game_index: Option<i32>,
    pub version: Option<NamedApiResource>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonHeldItem {
    pub item: Option<NamedApiResource>,
    pub version_details: Option<Vec<PokemonHeldItemVersion>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonHeldItemVersion {
    pub rarity: Option<i32>,
    pub version: Option<NamedApiResource>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMove {
    #[serde(rename = "move")]
    pub de_move: Option<NamedApiResource>,
    pub version_group_details: Option<Vec<PokemonMoveVersion>>,
}

impl PokemonMove {
    fn to_renderable(&self, gg: &str) -> Vec<RenderableMove> {
        let mut name = "";

        if let Some(mv) = self.de_move.as_ref() {
            if let Some(nm) = &mv.name {
                name = nm
            }
        }

        let prepared_moves: Vec<RenderableMove> = self
            .version_group_details
            .as_ref()
            .unwrap()
            .iter()
            .filter(|pmv| {
                if let Some(version_group) = &pmv.version_group {
                    return version_group.name.as_ref().unwrap_or(&String::new()) == gg
                }

                false
            })
            .map(|vgd| vgd.to_renderable(name.to_string()))
            .collect();

        prepared_moves
    }
}

#[derive(Tabled, Eq, Ord, PartialEq, PartialOrd, Debug)]
struct RenderableMove<'a> {
    #[header("Name")]
    name: String,
    #[header("Move Learn Method")]
    move_learn_method: &'a str,
    #[header("Level Learned At")]
    level_learned_at: i32,
}

impl PokemonMoveVersion {
    fn to_renderable(&self, name: String) -> RenderableMove {
        let mut move_learn_method = "";

        if let Some(mlm) = self.move_learn_method.as_ref() {
            if let Some(nm) = &mlm.name {
                move_learn_method = nm
            }
        }

        RenderableMove {
            move_learn_method,
            level_learned_at: self.level_learned_at.unwrap_or(0),
            name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMoveVersion {
    pub move_learn_method: Option<NamedApiResource>,
    pub version_group: Option<NamedApiResource>,
    pub level_learned_at: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonSprites {
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny_female: Option<String>,
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny_female: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonStat {
    pub stat: Option<NamedApiResource>,
    pub effort: Option<i32>,
    pub base_stat: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonType {
    pub slot: Option<i32>,
    #[serde(rename = "type")]
    pub de_type: Option<NamedApiResource>,
}