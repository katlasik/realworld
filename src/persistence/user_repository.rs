use crate::app_error::AppError;
use crate::database::Database;
use crate::model::indexed_user_field::IndexedUserField;
use crate::model::persistence::user::User;
use crate::persistence::params::insert_user_params::InsertUserParams;
use crate::persistence::params::update_user_params::UpdateUserParams;
use crate::persistence::schema::Users;
use anyhow::Result;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;

#[derive(Clone)]
pub struct UserRepository {
    database: Database,
}

impl UserRepository {
    pub fn new(database: Database) -> Self {
        UserRepository { database }
    }

    pub(crate) async fn insert_user(&self, params: InsertUserParams) -> Result<User, AppError> {
        let (sql, values) = Query::insert()
            .into_table(Users::Table)
            .columns([Users::Email, Users::Username, Users::PasswordHash])
            .values_panic([
                params.email.into(),
                params.username.into(),
                params.password_hash.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row = sqlx::query_with(&sql, values)
            .fetch_one(self.database.pool())
            .await?;

        Ok(User::from_row(row))
    }

    pub(crate) async fn update_user(&self, params: UpdateUserParams) -> Result<User, AppError> {
        let updates = params.as_list();

        if updates.is_empty() {
            return Err(AppError::BadData("No fields to update".to_string()));
        }

        let mut query = Query::update();
        query.table(Users::Table);

        for (column, value) in updates {
            query.value(column, value);
        }

        let (sql, values) = query
            .and_where(Expr::col(Users::Id).eq(params.user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row = sqlx::query_with(&sql, values)
            .fetch_one(self.database.pool())
            .await?;

        Ok(User::from_row(row))
    }

    pub(crate) async fn get_user_by<T>(
        &self,
        field: IndexedUserField,
        value: T,
    ) -> Result<Option<User>, AppError>
    where
        sea_query::Value: From<T>,
    {
        let field_name = field.to_field_name();

        let (sql, values) = Query::select()
            .column(Users::Id)
            .column(Users::Email)
            .column(Users::Username)
            .column(Users::PasswordHash)
            .column(Users::Bio)
            .column(Users::Image)
            .from(Users::Table)
            .and_where(Expr::col(field_name).eq(value))
            .build_sqlx(PostgresQueryBuilder);

        let row = sqlx::query_with(&sql, values)
            .fetch_optional(self.database.pool())
            .await?;

        Ok(row.map(User::from_row))
    }
}
