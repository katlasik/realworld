use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, put};
use tracing::info;
use crate::http::dto::user::{User, UserResponse, UpdateUserRequest};

pub(crate) fn user_routes() -> Router {
  Router::new()
    .route("/user", get(get_current_user))
    .route("/user", put(update_user))
}

async fn get_current_user() -> Result<Json<UserResponse>, StatusCode> {
  info!("Get current user");

  // TODO: Extract user from JWT token
  let user = User {
    email: "current@user.com".to_string().try_into().unwrap(),
    token: "mock.jwt.token".to_string(),
    username: "currentuser".into(),
    bio: Some("Current user bio".to_string()),
    image: None,
  };

  Ok(Json(UserResponse { user }))
}

async fn update_user(
  Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
  info!("Update user");

  // TODO: Extract user from JWT and update in database
  let user = User {
    email: payload.user.email.unwrap_or("updated@user.com".try_into().unwrap()),
    token: "mock.jwt.token".to_string(),
    username: payload.user.username.unwrap_or("updateduser".into()),
    bio: payload.user.bio,
    image: payload.user.image,
  };

  Ok(Json(UserResponse { user }))
}
