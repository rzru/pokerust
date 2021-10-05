use crate::named_api_resource::NamedApiResource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonAbility {
    pub is_hidden: Option<bool>,
    pub slot: Option<i32>,
    pub abilities: Option<Vec<NamedApiResource>>,
}
