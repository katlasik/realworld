use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::error;
use crate::http::dto::error::ErrorResponse;

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
            AppError::NotFound => (StatusCode::NOT_FOUND, Json::from(ErrorResponse::new("Not found".into()))).into_response(),

            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, Json::from(ErrorResponse::new("Unauthorized".into()))).into_response(),

            AppError::BadRequest(msg) => (StatusCode::UNPROCESSABLE_ENTITY, Json::from(ErrorResponse::new(msg))).into_response(),

            AppError::Db(err) => {
                error!("Database error: {err:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, Json::from(ErrorResponse::new("Database error".into()))).into_response()
            }

            AppError::Conflict(msg) => (StatusCode::CONFLICT, Json::from(ErrorResponse::new(msg))).into_response(),

            AppError::Other(err) => {
                error!("Internal: {err:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, Json::from(ErrorResponse::new("Internal server error".into()))).into_response()
            }
        }
    }
}
