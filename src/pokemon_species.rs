use crate::api_resource::APIResource;
use crate::flavor_text_entry::FlavorTextEntry;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonSpecies {
    pub gender_rate: Option<i32>,
    pub capture_rate: Option<i32>,
    pub base_happiness: Option<i32>,
    pub is_legendary: Option<bool>,
    pub evolution_chain: Option<APIResource>,
    pub flavor_text_entries: Option<Vec<FlavorTextEntry>>,
}

impl PokemonSpecies {
    pub fn gender_rate(&self) -> String {
        self.gender_rate.unwrap_or(0).to_string()
    }

    pub fn capture_rate(&self) -> String {
        self.capture_rate.unwrap_or(0).to_string()
    }

    pub fn base_happiness(&self) -> String {
        self.base_happiness.unwrap_or(0).to_string()
    }

    pub fn is_legendary(&self) -> String {
        self.is_legendary.unwrap_or(false).to_string()
    }

    pub fn description(&self) -> String {
        let mut description = String::new();

        if let Some(flavor_text_entries) = self.flavor_text_entries.as_ref() {
            let filtered_entries: Vec<&FlavorTextEntry> = flavor_text_entries
                .iter()
                .filter(|fte| {
                    let mut ok = false;

                    if let Some(language) = &fte.language {
                        ok = language.name.as_ref().unwrap_or(&String::new()) == "en"
                    }

                    ok
                })
                .collect();

            let found_entry = filtered_entries.first();
            if let Some(found_entry) = found_entry {
                description = found_entry
                    .flavor_text
                    .as_ref()
                    .unwrap_or(&String::new())
                    .to_string()
            }
        }

        description
    }
}
