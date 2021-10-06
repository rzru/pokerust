use crate::named_api_resource::NamedApiResource;
use serde::{Deserialize, Serialize};
use owo_colors::OwoColorize;

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonStat {
    pub stat: Option<NamedApiResource>,
    pub effort: Option<i32>,
    pub base_stat: Option<i32>,
}

impl PokemonStat {
    pub fn to_string(&self) -> String {
        let stat_name = self.stat.as_ref().unwrap().name.as_ref().unwrap();
        let base_stat = self.base_stat.unwrap_or(0);

        format!("{}: {}", stat_name.bold().green(), base_stat)
    }
}