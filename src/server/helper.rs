use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::api;

impl IntoResponse for api::ApiError {
    fn into_response(self) -> Response {
        match self {
            api::ApiError::AxumError(_)
            | api::ApiError::IoError(_)
            | api::ApiError::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            _ => (StatusCode::BAD_REQUEST, "TODO".to_string()).into_response(),
        }
    }
}
