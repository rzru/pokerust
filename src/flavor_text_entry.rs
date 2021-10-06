use crate::named_api_resource::NamedApiResource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FlavorTextEntry {
    pub flavor_text: Option<String>,
    pub version_group: Option<NamedApiResource>,
    pub language: Option<NamedApiResource>,
}
