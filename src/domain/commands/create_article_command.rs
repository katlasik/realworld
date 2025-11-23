use crate::http::dto::article::CreateArticleRequest;
use crate::model::values::article_body::ArticleBody;
use crate::model::values::article_description::ArticleDescription;
use crate::model::values::article_title::ArticleTitle;
use crate::model::values::slug::Slug;
use crate::model::values::tag_name::TagName;
use crate::model::values::user_id::UserId;
use crate::persistence::params::insert_article_params::InsertArticleParams;

#[derive(Debug, Clone)]
pub struct CreateArticleCommand {
    pub title: ArticleTitle,
    pub description: ArticleDescription,
    pub body: ArticleBody,
    pub tag_list: Vec<TagName>,
    pub author_id: UserId,
}

impl CreateArticleCommand {
    pub fn from_request(dto: CreateArticleRequest, author_id: UserId) -> Self {
        CreateArticleCommand {
            title: dto.article.title,
            description: dto.article.description,
            body: dto.article.body,
            tag_list: dto.article.tag_list.unwrap_or_default(),
            author_id,
        }
    }

    pub fn to_insert_params(&self, slug: Slug) -> InsertArticleParams {
        InsertArticleParams {
            slug,
            title: self.title.clone(),
            description: self.description.clone(),
            body: self.body.clone(),
            author_id: self.author_id,
        }
    }
}
