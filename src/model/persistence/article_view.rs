use chrono::{DateTime, Utc};
use sqlx::Row;
use crate::model::values::article_body::ArticleBody;
use crate::model::values::article_description::ArticleDescription;
use crate::model::values::article_id::ArticleId;
use crate::model::values::article_title::ArticleTitle;
use crate::model::values::bio::Bio;
use crate::model::values::image::Image;
use crate::model::values::slug::Slug;
use crate::model::values::tag_name::TagName;
use crate::model::values::user_id::UserId;
use crate::model::values::username::Username;

pub struct ArticleView {
  pub id: ArticleId,
  pub slug: Slug,
  pub title: ArticleTitle,
  pub description: ArticleDescription,
  pub tag_list: Vec<TagName>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub favorited: bool,
  pub favorites_count: i64,
  pub author_id: UserId,
  pub author: Username,
  pub author_bio: Option<Bio>,
  pub author_image: Option<Image>,
  pub following: bool,
  pub body: ArticleBody
}

impl ArticleView {

  pub fn from_row(row: sqlx::postgres::PgRow) -> ArticleView {
    ArticleView {
      id: row.get("id"),
      slug: row.get("slug"),
      title: row.get("title"),
      description: row.get("description"),
      tag_list: row.get("tag_list"),
      created_at: row.get("created_at"),
      updated_at: row.get("updated_at"),
      favorited: row.get("favorited"),
      favorites_count: row.get("favorites_count"),
      author_id: row.get("author_id"),
      author: row.get("author_username"),
      author_bio: row.get("author_bio"),
      author_image: row.get("author_image"),
      following: row.get("following"),
      body: row.get("body"),
    }
  }
}

pub struct ArticleListView {
  pub slug: Slug,
  pub title: ArticleTitle,
  pub description: ArticleDescription,
  pub tag_list: Vec<TagName>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub favorited: bool,
  pub favorites_count: i64,
  pub author: Username,
  pub author_bio: Option<Bio>,
  pub author_image: Option<Image>,
  pub following: bool,
}

impl ArticleListView {

  pub fn from_row(row: sqlx::postgres::PgRow) -> ArticleListView {
    ArticleListView {
      slug: row.get("slug"),
      title: row.get("title"),
      description: row.get("description"),
      tag_list: row.get("tag_list"),
      created_at: row.get("created_at"),
      updated_at: row.get("updated_at"),
      favorited: row.get("favorited"),
      favorites_count: row.get("favorites_count"),
      author: row.get("author_username"),
      author_bio: row.get("author_bio"),
      author_image: row.get("author_image"),
      following: row.get("following"),
    }
  }
}