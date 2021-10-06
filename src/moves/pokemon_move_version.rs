use super::{PokemonMoveExt, RenderableMove};
use crate::named_api_resource::NamedApiResource;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMoveVersion {
    pub move_learn_method: Option<NamedApiResource>,
    pub version_group: Option<NamedApiResource>,
    pub level_learned_at: Option<i32>,
}

impl PokemonMoveVersion {
    fn move_learn_method(&self) -> &str {
        let mut move_learn_method = "";
        if let Some(mlm) = self.move_learn_method.as_ref() {
            if let Some(nm) = &mlm.name {
                move_learn_method = nm
            }
        }

        move_learn_method
    }

    pub fn to_renderable(
        &self,
        name: String,
        ext: Option<&PokemonMoveExt>,
        gg: &str,
    ) -> RenderableMove {
        let mut accuracy = 0;
        let mut pp = 0;
        let mut power = 0;
        let mut pk_type = String::new();
        let mut description = String::new();

        if let Some(ext) = ext {
            accuracy = ext.accuracy();
            pp = ext.pp();
            power = ext.power();
            pk_type = ext.pk_type();
            description = ext.description(gg);
        }

        RenderableMove {
            move_learn_method: self.move_learn_method(),
            level_learned_at: self.level_learned_at.unwrap_or(0),
            name: name.bold().to_string(),
            accuracy,
            power,
            pk_type,
            pp,
            description,
        }
    }
}
