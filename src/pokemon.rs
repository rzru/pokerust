use serde::{Serialize, Deserialize};
use crate::http::Http;
use crate::{BASE_URL};
use std::io;
use tabled::{Tabled, Table, Header, Modify, Full, Alignment, Disable};

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


    fn render_moves(&self) {
        if let Some(moves) = self.moves.as_ref() {
            let prepared_moves: Vec<RenderableMove> = moves
                .iter()
                .map(|mv| {
                    mv.to_renderable()
                })
                .collect();

            let table = Table::new(&prepared_moves)
                .with(Header("MOVES"))
                .to_string();

            println!("{}", &table);
        }
    }

    pub fn render(&self, should_render_moves: bool) -> io::Result<()> {
        let base_info = Table::new(
            vec![
                ("ID", &self.id.unwrap().to_string()),
                ("Name", self.name.as_ref().unwrap()),
                ("Base Experience", &self.base_experience.unwrap().to_string()),
                ("Height", &self.height.unwrap().to_string()),
                ("Weight", &self.weight.unwrap().to_string()),
            ]
        )
            .with(Disable::Row(..1))
            .with(Modify::new(Full)
                .with(Alignment::left())
                .with(Alignment::center_vertical())
            )
            .to_string();
        println!("{}", base_info);
        if should_render_moves {
            self.render_moves()
        }

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
    fn to_renderable(&self) -> RenderableMove {
        let mut name = "";

        if let Some(mv) = self.de_move.as_ref() {
            if let Some(nm) = &mv.name {
                name = nm
            }
        }

        let prepared_moves_versions: Vec<RenderableMoveVersion> = self
            .version_group_details
            .as_ref()
            .unwrap()
            .iter()
            .map(|vgd| vgd.to_renderable())
            .collect();

        let move_versions = Table::new(prepared_moves_versions)
            .to_string();

        RenderableMove {
            name,
            move_versions,
        }
    }
}

#[derive(Tabled)]
struct RenderableMove<'a> {
    #[header("Name")]
    name: &'a str,
    #[header("Move Versions")]
    move_versions: String,
}

#[derive(Tabled)]
struct RenderableMoveVersion<'a> {
    #[header("Move Learn Method")]
    move_learn_method: &'a str,
    #[header("Version Group")]
    version_group: &'a str,
    #[header("Level Learned At")]
    level_learned_at: String,
}

impl PokemonMoveVersion {
    fn to_renderable(&self) -> RenderableMoveVersion {
        let mut move_learn_method = "";
        let mut version_group = "";

        if let Some(vg) = self.version_group.as_ref() {
            if let Some(nm) = &vg.name {
                version_group = nm
            }
        }

        if let Some(mlm) = self.move_learn_method.as_ref() {
            if let Some(nm) = &mlm.name {
                move_learn_method = nm
            }
        }

        let level_learned_at = if let Some(level_learned_at) = self.level_learned_at { level_learned_at.to_string() } else { "?".to_string() };

        RenderableMoveVersion {
            version_group,
            move_learn_method,
            level_learned_at,
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