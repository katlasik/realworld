use anyhow::Context;
use crate::app_error::AppError;
use crate::http::AppState;
use crate::http::dto::user::{UpdateUserRequest, UserData, UserResponse};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, put};
use axum::{Json, Router};
use tracing::info;
use uuid::Uuid;
use crate::http::extractors::auth_token::AuthToken;

pub(crate) fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/user", get(get_current_user))
        .route("/user", put(update_user))
}

async fn get_current_user(
    State(app_state): State<AppState>,
    auth_user: AuthToken,
) -> Result<Json<UserResponse>, AppError> {


    let token = auth_user.value();

    let user_id = app_state
        .jwt_generator
        .verify_token(token)?
        .sub
        .parse::<Uuid>()
        .context("Invalid user ID in token")?;

    info!("Get current user: {}", user_id);

    let (user, token) = app_state
        .user_service
        .get_user_by_id(user_id)
        .await?;

    let user = UserData {
        email: user.email,
        token,
        username: user.username,
        bio: user.bio,
        image: user.image,
    };

    Ok(Json(UserResponse { user }))
}

async fn update_user(
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    info!("Update user");

    // TODO: Extract user from JWT and update in database
    let user = UserData {
        email: payload
            .user
            .email
            .unwrap_or("updated@user.com".try_into().unwrap()),
        token: "mock.jwt.token".to_string(),
        username: payload
            .user
            .username
            .unwrap_or("updateduser".try_into().unwrap()),
        bio: payload.user.bio,
        image: payload.user.image,
    };

    Ok(Json(UserResponse { user }))
}
