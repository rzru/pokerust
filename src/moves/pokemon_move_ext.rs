use crate::named_api_resource::NamedApiResource;
use crate::pk_type::Type;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonMoveExt {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub accuracy: Option<i32>,
    pub pp: Option<i32>,
    pub power: Option<i32>,
    #[serde(rename = "type")]
    pub pk_type: Option<NamedApiResource>,
    pub flavor_text_entries: Option<Vec<MoveFlavorText>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveFlavorText {
    pub flavor_text: Option<String>,
    pub version_group: Option<NamedApiResource>,
    pub language: Option<NamedApiResource>,
}

impl PokemonMoveExt {
    pub fn accuracy(&self) -> i32 {
        if let Some(accuracy) = self.accuracy {
            accuracy
        } else {
            0
        }
    }

    pub fn pp(&self) -> i32 {
        if let Some(pp) = self.pp {
            pp
        } else {
            0
        }
    }

    pub fn power(&self) -> i32 {
        if let Some(power) = self.pp {
            power
        } else {
            0
        }
    }

    pub fn pk_type(&self) -> String {
        let mut res = String::from("normal");

        if let Some(pk_type) = self.pk_type.as_ref() {
            if let Some(tp) = pk_type.name.as_ref() {
                res = Type::get(tp, tp).color_fmt()
            }
        }

        res
    }

    pub fn description(&self, gg: &str) -> String {
        let mut description = String::new();
        let descriptions: Vec<&MoveFlavorText> = self
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
}
