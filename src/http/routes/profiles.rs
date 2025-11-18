use axum::extract::Path;
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use tracing::info;
use crate::http::dto::profile::{Profile, ProfileResponse};
use crate::model::values::username::Username;

pub (crate) fn profile_routes() -> Router {
  Router::new()
    .route("/profiles/{username}", get(get_profile))
      .route("/profiles/{username}/follow", post(follow_user))
      .route("/profiles/{username}/follow", delete(unfollow_user))
}

async fn get_profile(
  Path(username): Path<Username>,
) -> Result<Json<ProfileResponse>, StatusCode> {
  info!("Get profile for username: {}", username);

  // TODO: Fetch profile from database
  let profile = Profile {
    username,
    bio: Some("User bio".to_string()),
    image: None,
    following: false,
  };

  Ok(Json(ProfileResponse { profile }))
}

async fn follow_user(
  Path(username): Path<Username>,
) -> Result<Json<ProfileResponse>, StatusCode> {
  info!("Follow user: {}", username);

  // TODO: Create follow relationship in database
  let profile = Profile {
    username,
    bio: Some("User bio".to_string()),
    image: None,
    following: true,
  };

  Ok(Json(ProfileResponse { profile }))
}

async fn unfollow_user(
  Path(username): Path<Username>,
) -> Result<Json<ProfileResponse>, StatusCode> {
  info!("Unfollow user: {}", username);

  // TODO: Remove follow relationship from database
  let profile = Profile {
    username,
    bio: Some("User bio".to_string()),
    image: None,
    following: false,
  };

  Ok(Json(ProfileResponse { profile }))
}