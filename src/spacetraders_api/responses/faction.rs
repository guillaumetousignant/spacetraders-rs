use super::Meta;
use super::Trait;
use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub headquarters: Waypoint,
    pub traits: Vec<Trait>,
    #[serde(rename = "isRecruiting")]
    pub is_recruiting: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionResponse {
    pub data: Faction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Factions {
    pub data: Vec<Faction>,
    pub meta: Meta,
}
