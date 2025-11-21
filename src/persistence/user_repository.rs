use crate::app_error::AppError;
use crate::database::Database;
use crate::model::persistence::user::User;
use anyhow::Result;
use sqlx::{Encode, Postgres, QueryBuilder, Type};
use crate::persistence::indexed_user_field::IndexedUserField;
use crate::persistence::params::insert_user_params::InsertUserParams;
use crate::persistence::params::update_user_params::UpdateUserParams;

#[derive(Clone)]
pub struct UserRepository {
    database: Database,
}

impl UserRepository {
    pub fn new(database: Database) -> Self {
        UserRepository { database }
    }

    pub(crate) async fn insert_user(
        &self,
        params: InsertUserParams,
    ) -> Result<User, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO users (email, username, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, email, username, password_hash, bio, image
            "#,
        )
        .bind(params.email)
        .bind(params.username)
        .bind(params.password_hash)
        .fetch_one(self.database.pool())
        .await?;

        Ok(User::from_row(row))
    }

    pub(crate) async fn update_user(
      &self,
      params: UpdateUserParams,
    ) -> Result<User, AppError> {
      let updates = params.as_list();

      if updates.is_empty() {
        Err(AppError::BadData("No fields to update".to_string()))
      } else {

        let mut query_builder: QueryBuilder<Postgres> = sqlx::QueryBuilder::new("UPDATE users SET ");

        let mut separated = query_builder.separated(", ");
        let mut idx = 1;
        for (field, _) in &updates {
          separated.push(format!("{field} = ${idx}"));
          idx += 1;
        }
        query_builder.push(format!(" WHERE id = ${idx} RETURNING id, email, username, password_hash, bio, image"));

        let mut sql_query = query_builder.build();


        for (_, value) in &updates {
          sql_query = sql_query.bind(value);
        }
        sql_query = sql_query.bind(params.user_id);

        let row = sql_query.fetch_one(self.database.pool()).await?;

        Ok(User::from_row(row))
      }
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
