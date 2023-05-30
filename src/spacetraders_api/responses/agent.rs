use super::AgentData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    pub data: AgentData,
}
