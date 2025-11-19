use crate::app_error::AppError;
use crate::domain::user_repository::{IndexedUserField, UserRepository};
use crate::model::persistence::user::User;
use crate::model::values::email::Email;
use anyhow::Result;
use crate::domain::commands::register_command::RegisterCommand;
use crate::utils::hasher::Hasher;
use crate::utils::jwt::JwtGenerator;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    hasher: Hasher,
    jwt: JwtGenerator,
}

impl UserService {
    pub fn new(user_repo: UserRepository, hasher: Hasher, jwt_service: JwtGenerator) -> Self {
        UserService {
            user_repo,
            hasher,
          jwt: jwt_service,
        }
    }

    pub async fn register_user(&self, request: RegisterCommand) -> Result<(User, String), AppError> {
        let password_hash = self.hasher.hash_password(&request.password)?;

        if self.user_repo.get_user_by(IndexedUserField::Username, request.username.clone()).await?.is_some() {
            return Err(AppError::Conflict(format!(
                "Username '{}' is already taken",
                request.username
            )));
        } else if self.user_repo.get_user_by(IndexedUserField::Email, request.email.clone()).await?.is_some() {
            return Err(AppError::Conflict(format!(
                "Email '{}' is already registered",
                request.email
            )));
        }

        let user = self
            .user_repo
            .insert_user(&request.email, &request.username, &password_hash)
            .await?;

        let token = self
            .jwt
            .generate_token(user.id)?;

        Ok((user, token))
    }

    pub async fn login_user(&self, email: Email, password: String) -> Result<(User, String), AppError> {
        let user = self
            .user_repo
            .get_user_by(IndexedUserField::Email, email.clone())
            .await?
            .ok_or_else(|| AppError::Unauthorized)?;

        let is_valid = self
            .hasher
            .verify_password(&password, &user.password_hash)?;

        if is_valid {
          let token = self
            .jwt
            .generate_token(user.id)?;
          Ok((user, token))
        } else {
            Err(AppError::Unauthorized)
        }
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<(User, String), AppError> {
        let user = self
            .user_repo
            .get_user_by(IndexedUserField::Id, user_id)
            .await?
            .ok_or(AppError::Unauthorized)?;

        let token = self
            .jwt
            .generate_token(user.id)?;

        Ok((user, token))
    }
}
