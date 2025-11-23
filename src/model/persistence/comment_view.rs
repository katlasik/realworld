use chrono::{DateTime, Utc};
use sqlx::Row;
use crate::model::values::bio::Bio;
use crate::model::values::comment_body::CommentBody;
use crate::model::values::comment_id::CommentId;
use crate::model::values::image::Image;
use crate::model::values::username::Username;

pub struct CommentView {
    pub id: CommentId,
    pub body: CommentBody,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: Username,
    pub author_bio: Option<Bio>,
    pub author_image: Option<Image>,
    pub following: bool,
}

impl CommentView {

    pub fn from_row(row: sqlx::postgres::PgRow) -> CommentView {
        CommentView {
            id: row.get("id"),
            body: row.get("body"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            author: row.get("author_username"),
            author_bio: row.get("author_bio"),
            author_image: row.get("author_image"),
            following: row.get("following"),
        }
    }
}
