use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Internal server error")]
    InternalError,

    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Search error: {0}")]
    SearchError(String),

    #[error("Subscription error: {0}")]
    SubscriptionError(String),

    #[error("Chrono error: {0}")]
    ChronoError(#[from] chrono::ParseError),

    #[cfg(feature = "server")]
    #[error("Axum error: {0}")]
    AxumError(#[from] axum::Error),

    #[cfg(feature = "server")]
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ApiError>;
