use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub service: String,
    pub version: String,
    pub commit: String,
    pub status: String,
    pub uptime: chrono::Duration,

    pub is_indexer: bool,
    pub is_parser: bool,
    pub is_classifier: bool,
    pub is_profiler: bool,
    pub is_scorer: bool,
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for HealthResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json(self)).into_response()
    }
}
