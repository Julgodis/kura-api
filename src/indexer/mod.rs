use serde::{Deserialize, Serialize};

mod recent;
mod search;

pub use recent::{RecentRequest, RecentResponse};
pub use search::{SearchRequest, SearchResponse};

use crate::release::Release;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseEntry {
    pub id: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub data: Release,
}

