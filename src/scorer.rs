use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::release::Release;
use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreRequest {
    pub release: Release,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreResponse {
    pub score: i64,
    #[serde(default)]
    pub score_map: HashMap<String, i64>,
}

impl ScoreRequest {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for ScoreResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json(self)).into_response()
    }
}