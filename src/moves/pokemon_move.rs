use super::{PokemonMoveExt, PokemonMoveVersion, RenderableMove};
use crate::named_api_resource::NamedApiResource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMove {
    #[serde(rename = "move")]
    pub de_move: Option<NamedApiResource>,
    pub version_group_details: Option<Vec<PokemonMoveVersion>>,
}

impl PokemonMove {
    pub fn to_renderable(&self, gg: &str, ext: Option<&PokemonMoveExt>) -> Vec<RenderableMove> {
        let mut name = "";

        if let Some(mv) = self.de_move.as_ref() {
            if let Some(nm) = &mv.name {
                name = nm
            }
        }

        let mut prepared_moves: Vec<RenderableMove> = vec![];

        if let Some(version_group_details) = self.version_group_details.as_ref() {
            prepared_moves = version_group_details
                .iter()
                .filter(|pmv| {
                    if let Some(version_group) = &pmv.version_group {
                        return version_group.name.as_ref().unwrap_or(&String::new()) == gg;
                    }

                    false
                })
                .map(|vgd| vgd.to_renderable(name.to_string(), ext, gg))
                .collect();
        }

        prepared_moves
    }
}
