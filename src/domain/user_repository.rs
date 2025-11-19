use crate::app_error::AppError;
use crate::database::Database;
use crate::model::persistence::user::User;
use anyhow::Result;
use sqlx::{Encode, Postgres, Type};

#[derive(Clone)]
pub struct UserRepository {
    database: Database,
}

pub enum IndexedUserField {
  Email,
  Username,
  Id
}

impl IndexedUserField {
  fn to_field_name(&self) -> &str {
      match self {
          IndexedUserField::Email => "email",
          IndexedUserField::Username => "username",
          IndexedUserField::Id => "id",
      }
  }
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



    pub(crate) async fn get_user_by<T>(&self, field: IndexedUserField, value: T) -> Result<Option<User>, AppError>
    where
        T: for<'a> Encode<'a, Postgres> + Type<Postgres> + Send,
    {


        let query = format!(
            r#"
            SELECT id, email, username, password_hash, bio, image
            FROM users
            WHERE {} = $1
            "#,
            field.to_field_name()
        );

        let row = sqlx::query(&query)
        .bind(value)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(row.map(User::from_row))
    }
}
