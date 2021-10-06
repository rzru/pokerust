use crate::flavor_text_entry::FlavorTextEntry;
use crate::named_api_resource::NamedApiResource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonAbilityExt {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub effect_entries: Option<Vec<VerboseEffect>>,
    pub flavor_text_entries: Option<Vec<FlavorTextEntry>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerboseEffect {
    pub effect: Option<String>,
    pub short_effect: Option<String>,
    pub language: Option<NamedApiResource>,
}

impl PokemonAbilityExt {
    pub fn name(&self) -> String {
        self.name.as_ref().unwrap_or(&String::new()).to_string()
    }

    pub fn description(&self, gg: &str) -> String {
        let mut description = String::new();
        let descriptions: Vec<&FlavorTextEntry> = self
            .flavor_text_entries
            .as_ref()
            .unwrap()
            .iter()
            .filter(|fte| {
                let mut ok = false;

                if let Some(version_group) = &fte.version_group {
                    ok = version_group.name.as_ref().unwrap_or(&String::new()) == gg
                }

                if let Some(language) = &fte.language {
                    ok = language.name.as_ref().unwrap_or(&String::new()) == "en"
                }

                ok
            })
            .collect();

        if let Some(flavor_vg) = descriptions.first() {
            if let Some(descr) = flavor_vg.flavor_text.as_ref() {
                description = descr.to_string()
            }
        }

        description
    }

    pub fn effects(&self) -> (String, String) {
        let mut effect = String::new();
        let mut short_effect = String::new();

        let effects: Vec<&VerboseEffect> = self
            .effect_entries
            .as_ref()
            .unwrap()
            .iter()
            .filter(|ee| {
                let mut ok = false;

                if let Some(language) = &ee.language {
                    ok = language.name.as_ref().unwrap_or(&String::new()) == "en"
                }

                ok
            })
            .collect();

        if let Some(verbose_effects) = effects.first() {
            if let Some(fetched_effect) = verbose_effects.effect.as_ref() {
                effect = fetched_effect.to_string()
            }

            if let Some(fetched_short_effect) = verbose_effects.short_effect.as_ref() {
                short_effect = fetched_short_effect.to_string()
            }
        }

        (effect, short_effect)
    }
}
