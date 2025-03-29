use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeRequest {
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeEvent {
}