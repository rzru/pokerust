use crate::named_api_resource::NamedApiResource;
use serde::{Deserialize, Serialize};
use tabled::Tabled;
use crate::pk_type::Type;

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
            .map(|vgd| vgd.to_renderable(name.to_string(), ext, gg))
            .collect();

        prepared_moves
    }
}

#[derive(Tabled, Eq, Ord, PartialEq, PartialOrd, Debug)]
pub struct RenderableMove<'a> {
    #[header("Name")]
    pub name: String,
    #[header("Type")]
    pub pk_type: String,
    #[header("Power")]
    pub power: i32,
    #[header("PP")]
    pub pp: i32,
    #[header("Accuracy")]
    pub accuracy: i32,
    #[header("Description")]
    pub flavor: String,
    #[header("Move Learn Method")]
    pub move_learn_method: &'a str,
    #[header("Level Learned At")]
    pub level_learned_at: i32,
}

impl PokemonMoveVersion {
    fn to_renderable(&self, name: String, ext: Option<&PokemonMoveExt>, gg: &str) -> RenderableMove {
        let mut move_learn_method = "";
        let mut accuracy = 0;
        let mut pp = 0;
        let mut power = 0;
        let mut pk_type = String::new();
        let mut flavor = String::new();

        if let Some(ext) = ext {
            accuracy = ext.accuracy.unwrap_or(0);
            pp = ext.pp.unwrap_or(0);
            power = ext.power.unwrap_or(0);
            if let Some(tp) = &ext.pk_type {
                let normal_type = String::from("normal");
                let type_name = tp.name.as_ref().unwrap_or(&normal_type);
                pk_type = Type::get(type_name, type_name).color_fmt();
            }
            let flavor_vg: Vec<&MoveFlavorText> = ext
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
            if let Some(flavor_vg) = flavor_vg.first() {
                flavor = flavor_vg.flavor_text.as_ref().unwrap().to_string()
            }
        }

        if let Some(mlm) = self.move_learn_method.as_ref() {
            if let Some(nm) = &mlm.name {
                move_learn_method = nm
            }
        }

        RenderableMove {
            move_learn_method,
            level_learned_at: self.level_learned_at.unwrap_or(0),
            name,
            accuracy,
            power,
            pk_type,
            pp,
            flavor,
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