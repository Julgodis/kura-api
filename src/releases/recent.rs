use serde::{Deserialize, Serialize};

use super::Release;
use crate::Error;
use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentRequest {
    pub since: chrono::DateTime<chrono::Utc>,
    pub count: Option<usize>,
}

impl RecentRequest {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentResponse {
    pub since: chrono::DateTime<chrono::Utc>,
    pub until: chrono::DateTime<chrono::Utc>,
    pub releases: Vec<Release>,
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for RecentResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json(self)).into_response()
    }
}