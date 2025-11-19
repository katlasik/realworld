use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Database error")]
    Db(#[from] sqlx::Error),
    #[error("Internal error: {0}")]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found").into_response(),

            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),

            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),

            AppError::Db(err) => {
                tracing::error!("Database error: {err:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }

            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg).into_response(),

            AppError::Other(err) => {
                tracing::error!("Internal: {err:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}
