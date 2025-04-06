use serde::{Deserialize, Serialize};

use crate::release::Release;
use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileRequest {
    pub release: Release,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileResponse {
    pub release: Release,
}

impl ProfileRequest {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for ProfileResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json(self)).into_response()
    }
}