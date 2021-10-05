use crate::named_api_resource::NamedApiResource;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMove {
    #[serde(rename = "move")]
    pub de_move: Option<NamedApiResource>,
    pub version_group_details: Option<Vec<PokemonMoveVersion>>,
}

impl PokemonMove {
    pub fn to_renderable(&self, gg: &str) -> Vec<RenderableMove> {
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
                    return version_group.name.as_ref().unwrap_or(&String::new()) == gg;
                }

                false
            })
            .map(|vgd| vgd.to_renderable(name.to_string()))
            .collect();

        prepared_moves
    }
}

#[derive(Tabled, Eq, Ord, PartialEq, PartialOrd, Debug)]
pub struct RenderableMove<'a> {
    #[header("Name")]
    pub name: String,
    #[header("Move Learn Method")]
    pub move_learn_method: &'a str,
    #[header("Level Learned At")]
    pub level_learned_at: i32,
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
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct PokemonMoveExt {
//     pub id: Option<i32>,
// }
