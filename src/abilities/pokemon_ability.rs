use crate::abilities::renderable_ability::RenderableAbility;
use crate::abilities::PokemonAbilityExt;
use crate::named_api_resource::NamedApiResource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonAbility {
    pub is_hidden: Option<bool>,
    pub slot: Option<i32>,
    pub ability: Option<NamedApiResource>,
}

impl PokemonAbility {
    fn is_hidden(&self) -> bool {
        if let Some(is_hidden) = self.is_hidden {
            is_hidden
        } else {
            false
        }
    }

    pub fn to_renderable(
        &self,
        gg: &str,
        ext: Option<&PokemonAbilityExt>,
    ) -> Option<RenderableAbility> {
        let is_hidden = self.is_hidden();
        let mut name = String::new();
        let mut effect = String::new();
        let mut short_effect = String::new();
        let mut description = String::new();

        if let Some(ext) = ext {
            name = ext.name();
            let (fetched_effect, fetched_short_effect) = ext.effects();
            effect = fetched_effect;
            short_effect = fetched_short_effect;
            description = ext.description(gg);
        }

        Some(RenderableAbility {
            is_hidden,
            name,
            effect,
            short_effect,
            description,
        })
    }
}
