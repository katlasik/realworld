use crate::http::dto::profile::Profile;
use crate::model::values::article_body::ArticleBody;
use crate::model::values::article_description::ArticleDescription;
use crate::model::values::article_title::ArticleTitle;
use crate::model::values::slug::Slug;
use crate::model::values::tag_name::TagName;
use crate::model::values::username::Username;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::model::limit::Limit;
use crate::model::offset::Offset;
use crate::model::persistence::article_view::{ArticleListView, ArticleView};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleResponse {
    pub article: ArticleItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticlesResponse {
    pub articles: Vec<ArticleListItem>,
    #[serde(rename = "articlesCount")]
    pub articles_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleItem {
    pub slug: Slug,
    pub title: ArticleTitle,
    pub description: ArticleDescription,
    pub body: ArticleBody,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<TagName>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: i64,
    pub author: Profile,
} 

impl ArticleItem {
  pub(crate) fn from_article_view(view: &ArticleView) -> ArticleItem {
    ArticleItem {
      slug: view.slug.clone(),
      title: view.title.clone(),
      description: view.description.clone(),
      body: view.body.clone(),
      tag_list: view.tag_list.clone(),
      created_at: view.created_at,
      updated_at: view.updated_at,
      favorited: view.favorited,
      favorites_count: view.favorites_count,
      author: Profile {
        username: view.author.clone(),
        bio: view.author_bio.clone(),
        image: view.author_image.clone(),
        following: view.following,
      },
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleListItem {
    pub slug: Slug,
    pub title: ArticleTitle,
    pub description: ArticleDescription,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<TagName>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: i64,
    pub author: Profile,
}

impl ArticleListItem {
  pub(crate) fn from_article_view(view: &ArticleListView) -> ArticleListItem {
    ArticleListItem {
      slug: view.slug.clone(),
      title: view.title.clone(),
      description: view.description.clone(),
      tag_list: view.tag_list.clone(),
      created_at: view.created_at,
      updated_at: view.updated_at,
      favorited: view.favorited,
      favorites_count: view.favorites_count,
      author: Profile {
        username: view.author.clone(),
        bio: view.author_bio.clone(),
        image: view.author_image.clone(),
        following: view.following,
      },
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArticleRequest {
    pub article: CreateArticle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArticle {
    pub title: ArticleTitle,
    pub description: ArticleDescription,
    pub body: ArticleBody,
    #[serde(rename = "tagList")]
    pub tag_list: Option<Vec<TagName>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateArticleRequest {
    pub article: UpdateArticle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateArticle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<ArticleTitle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<ArticleDescription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ArticleBody>,
}

#[derive(Deserialize)]
pub struct ArticleListQuery {
    pub tag: Option<TagName>,
    pub author: Option<Username>,
    pub favorited: Option<Username>,
    pub limit: Option<Limit>,
    pub offset: Option<Offset>,
}
