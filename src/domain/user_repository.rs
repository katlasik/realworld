use crate::app_error::AppError;
use crate::database::Database;
use crate::model::persistence::user::User;
use crate::model::values::email::Email;
use crate::model::values::username::Username;
use anyhow::Result;
use sqlx::error::ErrorKind;
use sqlx::postgres::PgRow;
use sqlx::{Error, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    database: Database,
}

enum IndexedUserField {
  Email,
  Username,
  Id
}

impl UserRepository {
    pub fn new(database: Database) -> Self {
        UserRepository { database }
    }

    pub(crate) async fn insert_user(
        &self,
        email: &str,
        username: &str,
        password_hash: &str,
    ) -> Result<User, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO users (email, username, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, email, username, password_hash, bio, image
            "#,
        )
        .bind(email)
        .bind(username)
        .bind(password_hash)
        .fetch_one(self.database.pool())
        .await?;

        Ok(User::from_row(row))
    }



    pub(crate) async fn get_user_by_username(&self, username: Username) -> Result<Option<User>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, email, username, password_hash, bio, image
            FROM users
            WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(row.map(User::from_row))
    }

    pub(crate) async fn get_user_by_email(&self, email: Email) -> Result<Option<User>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, email, username, password_hash, bio, image
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(row.map(User::from_row))
    }

    pub(crate) async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, email, username, password_hash, bio, image
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(row.map(User::from_row))
    }
}
