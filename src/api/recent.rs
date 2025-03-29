use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentRequest {
    pub offset: Option<usize>,
    pub count: Option<usize>,
    pub since: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentResponse {
    pub interval: std::time::Duration,
    pub offset: usize,
    pub count: usize,
    pub total: usize,
    pub items: Vec<RecentItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentItem {
    pub id: String,
    pub title: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub seeders: usize,
    pub leechers: usize,
    pub category: String,
    pub size: u64,
}
