use crate::ability::PokemonAbility;
use crate::held_item::PokemonHeldItem;
use crate::http::Http;
use crate::named_api_resource::NamedApiResource;
use crate::pk_move::{PokemonMove, RenderableMove};
use crate::sprites::PokemonSprites;
use crate::version_game_index::VersionGameIndex;
use crate::BASE_URL;
use serde::{Deserialize, Serialize};
use std::io;
use tabled::{Alignment, Disable, Full, Modify, Table};

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
                .with(
                    Modify::new(Full)
                        .with(Alignment::left())
                        .with(Alignment::center_vertical()),
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
            .with(
                Modify::new(Full)
                    .with(Alignment::left())
                    .with(Alignment::center_vertical()),
            )
            .to_string();

        println!("{}", base_info);

        Ok(())
    }
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
