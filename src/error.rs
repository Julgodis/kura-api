#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Internal server error")]
    InternalError,

    #[error("Custom error: {0}")]
    CustomError(String, String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ErrorDescription {
    pub code: String,
    pub message: String,
}

impl Error {
    #[cfg(feature = "axum")]
    pub fn status(&self) -> axum::http::StatusCode {
        match self {
            Error::InternalError => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Error::CustomError(_, _) => axum::http::StatusCode::BAD_REQUEST,
        }
    }

    pub fn description(&self) -> ErrorDescription {
        match self {
            Error::InternalError => ErrorDescription {
                code: "500".to_string(),
                message: "Internal server error".to_string(),
            },
            Error::CustomError(code, message) => ErrorDescription {
                code: code.clone(),
                message: message.clone(),
            },
        }
    }
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (self.status(), axum::Json(self.description())).into_response()
    }
}
