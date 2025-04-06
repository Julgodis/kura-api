use serde::{Deserialize, Serialize};

use super::ReleaseEntry;
use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
}

impl SearchRequest {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub releases: Vec<ReleaseEntry>,
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for SearchResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json(self)).into_response()
    }
}
