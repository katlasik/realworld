use crate::app_error::AppError;
use crate::domain::commands::login_command::LoginCommand;
use crate::domain::commands::register_command::RegisterCommand;
use crate::domain::commands::update_user_command::UpdateUserCommand;
use crate::model::indexed_user_field::IndexedUserField;
use crate::model::persistence::user::User;
use crate::model::values::user_id::UserId;
use crate::persistence::user_repository::UserRepository;
use crate::utils::hasher::Hasher;
use anyhow::Result;
use tracing::log::info;
use crate::model::values::username::Username;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    hasher: Hasher,
}

impl UserService {
    pub fn new(user_repo: UserRepository, hasher: Hasher) -> Self {
        UserService { user_repo, hasher }
    }

    pub async fn register_user(&self, command: RegisterCommand) -> Result<User, AppError> {
        let password_hash = self.hasher.hash_password(&command.password)?;

        if self
            .user_repo
            .get_user_by(IndexedUserField::Username, command.username.clone())
            .await?
            .is_some()
        {
            return Err(AppError::DataConflict(format!(
                "Username '{}' is already taken",
                command.username
            )));
        } else if self
            .user_repo
            .get_user_by(IndexedUserField::Email, command.email.clone())
            .await?
            .is_some()
        {
            return Err(AppError::DataConflict(format!(
                "Email '{}' is already registered",
                command.email
            )));
        }

        let params = command.to_params(password_hash);
        let user = self.user_repo.insert_user(params).await?;

        Ok(user)
    }

    pub async fn login_user(&self, command: LoginCommand) -> Result<User, AppError> {
        let user = self
            .user_repo
            .get_user_by(IndexedUserField::Email, command.email.clone())
            .await?
            .ok_or_else(|| AppError::Unauthorized)?;

        if self
            .hasher
            .verify_password(&command.password, &user.password_hash)
            .map_err(|_| AppError::Unauthorized)?
        {
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

    pub async fn get_user_by<T>(
        &self,
        field: IndexedUserField,
        value: T,
    ) -> Result<Option<User>, AppError>
    where
        sea_query::Value: From<T>,
    {
        self.user_repo.get_user_by(field, value).await
    }

    pub async fn get_user_by_username(
        &self,
        username: Username
    ) -> Result<Option<User>, AppError>
    {
        self.user_repo.get_user_by(IndexedUserField::Username, username).await
    }

    pub(crate) async fn update_user(&self, command: UpdateUserCommand) -> Result<User, AppError> {
        let password_hash = command
            .password
            .as_ref()
            .map(|pw| self.hasher.hash_password(pw))
            .transpose()?;

        let params = command.to_params(password_hash);
        let user = self.user_repo.update_user(params).await?;

        info!("Updated user with id: {}", user.id);

        Ok(user)
    }
}
