use crate::named_api_resource::NamedApiResource;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonType {
    pub slot: Option<i32>,
    #[serde(rename = "type")]
    pub de_type: Option<NamedApiResource>,
}

pub enum Type<'a> {
    Normal(&'a str),
    Fire(&'a str),
    Water(&'a str),
    Electric(&'a str),
    Grass(&'a str),
    Ice(&'a str),
    Fighting(&'a str),
    Poison(&'a str),
    Ground(&'a str),
    Flying(&'a str),
    Psychic(&'a str),
    Bug(&'a str),
    Rock(&'a str),
    Ghost(&'a str),
    Dragon(&'a str),
    Dark(&'a str),
    Steel(&'a str),
    Fairy(&'a str),
}

impl<'a> Type<'a> {
    pub fn get(pk_type: &'a str, val: &'a str) -> Self {
        match pk_type {
            "normal" => Type::Normal(val),
            "fire" => Type::Fire(val),
            "water" => Type::Water(val),
            "electric" => Type::Electric(val),
            "grass" => Type::Grass(val),
            "ice" => Type::Ice(val),
            "fighting" => Type::Fighting(val),
            "poison" => Type::Poison(val),
            "ground" => Type::Ground(val),
            "flying" => Type::Flying(val),
            "psychic" => Type::Psychic(val),
            "bug" => Type::Bug(val),
            "rock" => Type::Rock(val),
            "ghost" => Type::Ghost(val),
            "dragon" => Type::Dragon(val),
            "dark" => Type::Dark(val),
            "steel" => Type::Steel(val),
            "fairy" => Type::Fairy(val),
            _ => Type::Normal(val),
        }
    }

    pub fn color_fmt(&self) -> String {
        match self {
            Type::Normal(val) => val.bold().truecolor(170, 170, 153).to_string(),
            Type::Fire(val) => val.bold().truecolor(255, 68, 34).to_string(),
            Type::Water(val) => val.bold().truecolor(51, 153, 255).to_string(),
            Type::Electric(val) => val.bold().truecolor(255, 204, 51).to_string(),
            Type::Grass(val) => val.bold().truecolor(119, 204, 85).to_string(),
            Type::Ice(val) => val.bold().truecolor(102, 204, 255).to_string(),
            Type::Fighting(val) => val.bold().truecolor(187, 85, 68).to_string(),
            Type::Poison(val) => val.bold().truecolor(170, 85, 153).to_string(),
            Type::Ground(val) => val.bold().truecolor(221, 187, 85).to_string(),
            Type::Flying(val) => val.bold().truecolor(136, 153, 255).to_string(),
            Type::Psychic(val) => val.bold().truecolor(255, 85, 153).to_string(),
            Type::Bug(val) => val.bold().truecolor(170, 187, 34).to_string(),
            Type::Rock(val) => val.bold().truecolor(187, 170, 102).to_string(),
            Type::Ghost(val) => val.bold().truecolor(102, 102, 187).to_string(),
            Type::Dragon(val) => val.bold().truecolor(119, 102, 238).to_string(),
            Type::Dark(val) => val.bold().truecolor(119, 85, 68).to_string(),
            Type::Steel(val) => val.bold().truecolor(170, 170, 187).to_string(),
            Type::Fairy(val) => val.bold().truecolor(238, 153, 238).to_string(),
        }
    }
}
