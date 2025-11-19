use axum::{
  extract::{FromRequestParts},
  http::{request::Parts, StatusCode},
};

pub struct AuthToken(String);

impl AuthToken {
  pub fn value(&self) -> &str {
    &self.0
  }
}

impl<S> FromRequestParts<S> for AuthToken
where
  S: Send + Sync,
{
  type Rejection = (StatusCode, &'static str);

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    let auth = parts
      .headers
      .get(axum::http::header::AUTHORIZATION)
      .and_then(|h| h.to_str().ok())
      .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?;

    let token = auth
      .strip_prefix("Bearer ")
      .ok_or((StatusCode::UNAUTHORIZED, "Invalid Bearer token"))?;

    Ok(AuthToken(token.to_string()))
  }

}
