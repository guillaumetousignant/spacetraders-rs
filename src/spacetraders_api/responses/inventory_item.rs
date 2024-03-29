use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub units: u128,
}
