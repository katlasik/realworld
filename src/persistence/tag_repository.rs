use crate::app_error::AppError;
use crate::database::Database;
use crate::model::persistence::tag::Tag;
use crate::model::values::tag_name::TagName;
use crate::persistence::params::insert_tag_params::InsertTagParams;
use crate::persistence::schema::Tags;
use anyhow::Result;
use sea_query::{Expr, Order, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::Row;

#[derive(Clone)]
pub struct TagRepository {
    database: Database,
}

impl TagRepository {
    pub fn new(database: Database) -> Self {
        TagRepository { database }
    }

    pub async fn insert_tag(&self, params: InsertTagParams) -> Result<Tag, AppError> {
        let (sql, values) = Query::insert()
            .into_table(Tags::Table)
            .columns([Tags::Name])
            .values_panic([params.name.into()])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row = sqlx::query_with(&sql, values)
            .fetch_one(self.database.pool())
            .await?;

        Ok(Tag::from_row(row))
    }

    pub async fn get_tag_by_name(&self, name: &TagName) -> Result<Option<Tag>, AppError> {
        let (sql, values) = Query::select()
            .columns([Tags::Id, Tags::Name, Tags::CreatedAt])
            .from(Tags::Table)
            .and_where(Expr::col(Tags::Name).eq(name))
            .build_sqlx(PostgresQueryBuilder);

        let row = sqlx::query_with(&sql, values)
            .fetch_optional(self.database.pool())
            .await?;

        Ok(row.map(Tag::from_row))
    }

    pub async fn get_all_tags(&self) -> Result<Vec<TagName>, AppError> {
        let (sql, values) = Query::select()
            .column(Tags::Name)
            .from(Tags::Table)
            .order_by(Tags::Name, Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        let rows = sqlx::query_with(&sql, values)
            .fetch_all(self.database.pool())
            .await?;

        Ok(rows.into_iter().map(|row| row.get("name")).collect())
    }

    pub async fn get_or_create_tag(&self, name: &TagName) -> Result<Tag, AppError> {
        if let Some(tag) = self.get_tag_by_name(name).await? {
            Ok(tag)
        } else {
            self.insert_tag(InsertTagParams { name: name.clone() })
                .await
        }
    }
}
