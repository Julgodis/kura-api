
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub term: String,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
}

