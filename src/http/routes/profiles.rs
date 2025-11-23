use crate::app_error::AppError;
use crate::http::AppState;
use crate::http::dto::profile::{Profile, ProfileResponse};
use crate::http::extractors::auth_token::AuthToken;
use crate::model::values::username::Username;
use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use tracing::info;

pub(crate) fn profile_routes() -> Router<AppState> {
    Router::new()
        .route("/profiles/{username}", get(get_profile))
        .route("/profiles/{username}/follow", post(follow_user))
        .route("/profiles/{username}/follow", delete(unfollow_user))
}

async fn get_profile(
    State(state): State<AppState>,
    auth: Option<AuthToken>,
    Path(username): Path<Username>,
) -> Result<Json<ProfileResponse>, AppError> {
    info!("Get profile for username: {}", username);

    let user = state
        .user_service
        .get_user_by_username( username.clone())
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    let following = if let Some(token) = auth {
        state
            .profile_service
            .is_following(token.user_id, user.id)
            .await?
    } else {
        false
    };

    let profile = Profile::from_user(user, following);

    Ok(Json(ProfileResponse { profile }))
}

async fn follow_user(
    State(state): State<AppState>,
    auth: AuthToken,
    Path(username): Path<Username>,
) -> Result<Json<ProfileResponse>, AppError> {
    info!("Follow user: {}", username);

    let user = state
        .user_service
        .get_user_by_username(username)
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    state
        .profile_service
        .follow_user(auth.user_id, user.id)
        .await?;

    let profile = Profile::from_user(user, true);

    Ok(Json(ProfileResponse { profile }))
}

async fn unfollow_user(
    State(state): State<AppState>,
    auth: AuthToken,
    Path(username): Path<Username>,
) -> Result<Json<ProfileResponse>, AppError> {
    info!("Unfollow user: {}", username);

    let user = state
        .user_service
        .get_user_by_username(username)
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    state
        .profile_service
        .unfollow_user(auth.user_id, user.id)
        .await?;

    let profile = Profile::from_user(user, false);

    Ok(Json(ProfileResponse { profile }))
}
