use crate::app_error::AppError;
use crate::domain::commands::create_article_command::CreateArticleCommand;
use crate::domain::commands::get_feed_query::GetFeedQuery;
use crate::domain::commands::list_articles_query::ListArticlesQuery;
use crate::domain::commands::update_article_command::UpdateArticleCommand;
use crate::model::values::slug::Slug;
use crate::model::values::tag_name::TagName;
use crate::model::values::user_id::UserId;
use crate::persistence::article_repository::ArticleRepository;
use crate::persistence::tag_repository::TagRepository;
use anyhow::Result;
use crate::model::indexed_article_field::IndexedArticleField;
use crate::model::persistence::article_view::{ArticleListView, ArticleView};
use crate::persistence::params::list_articles_params::ListArticlesParams;

#[derive(Clone)]
pub struct ArticleService {
    article_repo: ArticleRepository,
    tag_repo: TagRepository,
}

impl ArticleService {
    pub fn new(article_repo: ArticleRepository, tag_repo: TagRepository) -> Self {
        ArticleService {
            article_repo,
            tag_repo,
        }
    }

    async fn verify_slug(&self, slug: &Slug) -> Result<(), AppError> {
      if self
        .article_repo
        .get_article_by(IndexedArticleField::Slug, &slug)
        .await?
        .is_some()
      {
        Err(AppError::DataConflict(format!(
          "Article with slug '{}' already exists",
          slug
        )))
      } else {
        Ok(())
      }
    }

    pub async fn create_article(&self, command: CreateArticleCommand) -> Result<ArticleView, AppError> {
        let slug = Slug::from_title(command.title.value());

        self.verify_slug(&slug).await?;

        let params = command.to_insert_params(slug);
        let article = self.article_repo.insert_article(params).await?;

        let tag_ids = self.get_or_create_tags(&command.tag_list).await?;
        self.article_repo
            .add_tags_to_article(article.id, &tag_ids)
            .await?;

        let article_view = self.article_repo.get_article_by_id(article.id, Some(command.author_id)).await?;
        Ok(article_view)
    }

    pub async fn get_article(&self, slug: &Slug, user_id: Option<UserId>) -> Result<Option<ArticleView>, AppError> {
        self.article_repo.get_article_view_by(IndexedArticleField::Slug, slug, user_id).await
    }

    pub async fn update_article(&self, command: UpdateArticleCommand, user_id: UserId) -> Result<ArticleView, AppError> {

          let article = self.article_repo.get_article_by(IndexedArticleField::Slug, &command.old_slug).await?
            .ok_or(AppError::NotFound)?;

          let params = command.to_params(article.id);

          if article.author_id != user_id {
            Err(AppError::Forbidden)
          } else {

            if let Some(ref slug) = params.slug {
              self.verify_slug(slug).await?;
            }
            
            let article = self.article_repo.update_article(params).await?;
            Ok(self.article_repo.get_article_by_id(article.id, Some(user_id)).await?)
          }
    }

    pub async fn delete_article(&self, slug: Slug, user_id: UserId) -> Result<(), AppError> {

       let article = self.article_repo.get_article_by(IndexedArticleField::Slug, &slug).await?;

        if let Some(article) = article {
          if article.author_id != user_id {
            Err(AppError::Forbidden)
          } else {
            self.article_repo.delete_article(article.id).await
          }
        } else {
          Err(AppError::NotFound)
        }
    }

    pub async fn list_articles(&self, query: ListArticlesQuery, user_id: Option<UserId>) -> Result<Vec<ArticleListView>, AppError> {

        self.article_repo
            .list_articles(
                ListArticlesParams::from_query(query, user_id)
            )
            .await
    }

    pub async fn count_articles(&self, query: ListArticlesQuery, user_id: Option<UserId>) -> Result<i64, AppError> {
        self.article_repo
            .count_articles(
                ListArticlesParams::from_query(query, user_id)
            )
            .await
    }

    pub async fn get_feed(&self, query: GetFeedQuery) -> Result<Vec<ArticleListView>, AppError> {

        self.article_repo
            .get_feed_articles(query.user_id, query.limit, query.offset)
            .await
    }

    pub async fn favorite_article(
        &self,
        user_id: UserId,
        slug: &Slug,
    ) -> Result<(), AppError> {

        let article = self.article_repo.get_article_by(IndexedArticleField::Slug, &slug).await?
            .ok_or(AppError::NotFound)?;

        self.article_repo
            .favorite_article(user_id, article.id)
            .await
    }

    pub async fn unfavorite_article(
        &self,
        user_id: UserId,
        slug: &Slug,
    ) -> Result<(), AppError> {

      let article = self.article_repo.get_article_by(IndexedArticleField::Slug, &slug).await?
        .ok_or(AppError::NotFound)?;

        self.article_repo
            .unfavorite_article(user_id, article.id)
            .await
    }

    async fn get_or_create_tags(&self, tag_names: &[TagName]) -> Result<Vec<uuid::Uuid>, AppError> {
        let mut tag_ids = Vec::new();

        for tag_name in tag_names {
            let tag_name = TagName::try_from(tag_name.as_str())
                .map_err(|e| AppError::BadData(format!("Invalid tag name: {}", e)))?;

            let tag = self.tag_repo.get_or_create_tag(&tag_name).await?;
            tag_ids.push(tag.id.value());
        }

        Ok(tag_ids)
    }
}
