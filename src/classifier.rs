use std::collections::HashMap;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::release::Release;
use crate::Error;
use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassifyRequest {
    pub release: Release,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassifyResponse {
    pub release: Release,
}

impl ClassifyRequest {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for ClassifyResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json(self)).into_response()
    }
}