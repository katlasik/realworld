use crate::model::values::article_body::ArticleBody;
use crate::model::values::article_description::ArticleDescription;
use crate::model::values::article_id::ArticleId;
use crate::model::values::article_title::ArticleTitle;
use crate::model::values::slug::Slug;
use crate::model::values::user_id::UserId;
use chrono::{DateTime, Utc};
use sqlx::Row;
use sqlx::postgres::PgRow;

pub struct Article {
    pub id: ArticleId,
    pub slug: Slug,
    pub title: ArticleTitle,
    pub description: ArticleDescription,
    pub body: ArticleBody,
    pub author_id: UserId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Article {
    pub fn from_row(row: PgRow) -> Self {
        Self {
            id: row.get("id"),
            slug: row.get("slug"),
            title: row.get("title"),
            description: row.get("description"),
            body: row.get("body"),
            author_id: row.get("author_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

}
