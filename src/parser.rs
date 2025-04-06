use serde::{Deserialize, Serialize};

use crate::release::Release;
use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ParseRequest {
    pub release: Release,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParseResponse {
    pub release: Release,
}

impl ParseResponse {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for ParseResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json(self)).into_response()
    }
}