use anyhow::Context;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Clone)]
pub struct JwtGenerator {
    secret: String,
}

impl JwtGenerator {
    pub fn new(secret: String) -> Self {
        JwtGenerator { secret }
    }

    pub fn generate_token(&self, user_id: Uuid) -> Result<String, AppError> {
        let now = Utc::now();
        let expiration = now + Duration::days(30);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        ).context("Failed to generate JWT Token")?;

      Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        ).map_err(|_| AppError::Unauthorized)?;

        Ok(token_data.claims)
    }
}
