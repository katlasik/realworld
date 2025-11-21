use crate::app_error::AppError;
use crate::persistence::user_repository::{UserRepository};
use crate::model::persistence::user::User;
use crate::model::values::email::Email;
use crate::model::values::password::Password;
use crate::model::values::password_hash::PasswordHash;
use crate::model::values::user_id::UserId;
use anyhow::Result;
use tracing::log::info;
use crate::domain::commands::register_command::RegisterCommand;
use crate::utils::hasher::Hasher;
use crate::domain::commands::update_user_command::UpdateUserCommand;
use crate::persistence::indexed_user_field::IndexedUserField;
use crate::persistence::params::insert_user_params::InsertUserParams;
use crate::persistence::params::update_user_params::UpdateUserParams;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    hasher: Hasher
}

impl UserService {
    pub fn new(user_repo: UserRepository, hasher: Hasher) -> Self {
        UserService {
            user_repo,
            hasher
        }
    }

    pub async fn register_user(&self, command: RegisterCommand) -> Result<User, AppError> {
        let password_hash = self.hasher.hash_password(&command.password)?;

        if self.user_repo.get_user_by(IndexedUserField::Username, command.username.clone()).await?.is_some() {
            return Err(AppError::DataConflict(format!(
                "Username '{}' is already taken",
                command.username
            )));
        } else if self.user_repo.get_user_by(IndexedUserField::Email, command.email.clone()).await?.is_some() {
            return Err(AppError::DataConflict(format!(
                "Email '{}' is already registered",
                command.email
            )));
        }

        let user = self
            .user_repo
            .insert_user(
              InsertUserParams {
                  email: command.email,
                  username: command.username,
                  password_hash: password_hash,
              }
            )
            .await?;

        Ok(user)
    }

    pub async fn login_user(&self, email: Email, password: Password) -> Result<User, AppError> {
        let user = self
            .user_repo
            .get_user_by(IndexedUserField::Email, email.clone())
            .await?
            .ok_or_else(|| AppError::Unauthorized)?;

        if self.hasher.verify_password(&password, &user.password_hash).map_err(|_| AppError::Unauthorized)? {
         Ok(user)
        } else {
          Err(AppError::Unauthorized)
        }


    }

    pub async fn get_user_by_id(&self, user_id: UserId) -> Result<Option<User>, AppError> {
        let user = self
            .user_repo
            .get_user_by(IndexedUserField::Id, user_id)
            .await?;

        Ok(user)
    }

  pub(crate) async fn update_user(&self, command: UpdateUserCommand) -> Result<User, AppError> {

    let user = self
      .user_repo
      .update_user(
        UpdateUserParams {
          user_id: command.user_id,
          email: command.email,
          username: command.username,
          password_hash: command.password.map(|pw| self.hasher.hash_password(&pw).map(PasswordHash::from)).transpose()?,
          bio: command.bio,
          image: command.image,
        }
      )
      .await?;

    info!("Updated user with id: {}", user.id);

    Ok(user)
  }
}
