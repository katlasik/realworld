use crate::http::dto::article::UpdateArticleRequest;
use crate::model::values::article_body::ArticleBody;
use crate::model::values::article_description::ArticleDescription;
use crate::model::values::article_id::ArticleId;
use crate::model::values::article_title::ArticleTitle;
use crate::model::values::slug::Slug;
use crate::persistence::params::update_article_params::UpdateArticleParams;

#[derive(Debug, Clone)]
pub struct UpdateArticleCommand {
    pub old_slug: Slug,
    pub new_slug: Option<Slug>,
    pub title: Option<ArticleTitle>,
    pub description: Option<ArticleDescription>,
    pub body: Option<ArticleBody>,
}

impl UpdateArticleCommand {

    pub fn from_request(dto: UpdateArticleRequest, slug: Slug) -> Self {

        let new_slug = dto.article.title.as_ref().map(|t| Slug::from_title(t.value()));

        UpdateArticleCommand {
            old_slug: slug,
            new_slug,
            title: dto.article.title,
            description: dto.article.description,
            body: dto.article.body,
        }
    }

    pub fn to_params(&self, article_id: ArticleId) -> UpdateArticleParams {

        UpdateArticleParams {
            article_id,
            slug: self.new_slug.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            body: self.body.clone(),
        }
    }
}
