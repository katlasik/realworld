use crate::app_error::AppError;
use crate::database::Database;
use crate::model::indexed_article_field::IndexedArticleField;
use crate::model::limit::Limit;
use crate::model::offset::Offset;
use crate::model::persistence::article::Article;
use crate::model::persistence::article_view::{ArticleListView, ArticleView};
use crate::model::values::article_id::ArticleId;
use crate::model::values::user_id::UserId;
use crate::model::values::username::Username;
use crate::persistence::params::insert_article_params::InsertArticleParams;
use crate::persistence::params::list_articles_params::ListArticlesParams;
use crate::persistence::params::update_article_params::UpdateArticleParams;
use crate::persistence::schema::{
    ArticleFavorites, ArticleTags, Articles, Tags, UserFollows, Users,
};
use anyhow::Result;
use sea_query::{Alias, Expr, Order, PostgresQueryBuilder, Query, SelectStatement};
use sea_query_binder::SqlxBinder;
use sqlx::Row;

#[derive(Clone)]
pub struct ArticleRepository {
    database: Database,
}

fn following_subquery(user_id: UserId) -> SelectStatement {
    Query::select()
        .expr(Expr::cust("1"))
        .from(UserFollows::Table)
        .and_where(
            Expr::col((UserFollows::Table, UserFollows::FollowerId))
                .eq(user_id)
                .and(
                    Expr::col((UserFollows::Table, UserFollows::FolloweeId))
                        .eq(Expr::col((Users::Table, Users::Id))),
                ),
        )
        .to_owned()
}

fn favorited_subquery(favorited_by_username: Username) -> SelectStatement {
    Query::select()
        .column((ArticleFavorites::Table, ArticleFavorites::ArticleId))
        .from(ArticleFavorites::Table)
        .inner_join(
            Users::Table,
            Expr::col((ArticleFavorites::Table, ArticleFavorites::UserId))
                .eq(Expr::col((Users::Table, Users::Id))),
        )
        .and_where(Expr::col((Users::Table, Users::Username)).eq(favorited_by_username))
        .to_owned()
}

fn build_article_view_query(
    user_id: Option<UserId>,
    mut where_statement: impl FnMut(&mut SelectStatement),
) -> SelectStatement {
    let mut query = Query::select();
    query
          .column((Articles::Table, Articles::Id))
          .column((Articles::Table, Articles::Slug))
          .column((Articles::Table, Articles::Title))
          .column((Articles::Table, Articles::Description))
          .column((Articles::Table, Articles::CreatedAt))
          .column((Articles::Table, Articles::UpdatedAt))
          .column((Articles::Table, Articles::Body))
          .expr_as(
            Expr::col((Users::Table, Users::Id)),
            Alias::new("author_id"),
          )
          .expr_as(
            Expr::col((Users::Table, Users::Username)),
            Alias::new("author_username"),
          )
          .expr_as(
            Expr::col((Users::Table, Users::Bio)),
            Alias::new("author_bio"),
          )
          .expr_as(
            Expr::col((Users::Table, Users::Image)),
            Alias::new("author_image"),
          )
          .expr_as(
            Expr::cust("COUNT(DISTINCT article_favorites.user_id)"),
            Alias::new("favorites_count"),
          )
          .expr_as(
            Expr::cust("COALESCE(ARRAY_AGG(tags.name ORDER BY tags.name ASC) FILTER (WHERE tags.name IS NOT NULL), ARRAY[]::text[])::text[]"),
            Alias::new("tag_list"),
          );

    match user_id {
        Some(user_id) => {
            let following_subquery = following_subquery(user_id);

            let favorited_subquery = Query::select()
                .expr(Expr::cust("1"))
                .from(ArticleFavorites::Table)
                .and_where(
                    Expr::col((ArticleFavorites::Table, ArticleFavorites::UserId))
                        .eq(user_id)
                        .and(
                            Expr::col((ArticleFavorites::Table, ArticleFavorites::ArticleId))
                                .eq(Expr::col((Articles::Table, Articles::Id))),
                        ),
                )
                .to_owned();

            query
                .expr_as(Expr::exists(following_subquery), Alias::new("following"))
                .expr_as(Expr::exists(favorited_subquery), Alias::new("favorited"));
        }
        None => {
            query
                .expr_as(Expr::cust("FALSE"), Alias::new("following"))
                .expr_as(Expr::cust("FALSE"), Alias::new("favorited"));
        }
    }

    query
        .from(Articles::Table)
        .inner_join(
            Users::Table,
            Expr::col((Articles::Table, Articles::AuthorId))
                .eq(Expr::col((Users::Table, Users::Id))),
        )
        .left_join(
            ArticleTags::Table,
            Expr::col((Articles::Table, Articles::Id))
                .eq(Expr::col((ArticleTags::Table, ArticleTags::ArticleId))),
        )
        .left_join(
            Tags::Table,
            Expr::col((ArticleTags::Table, ArticleTags::TagId))
                .eq(Expr::col((Tags::Table, Tags::Id))),
        )
        .left_join(
            ArticleFavorites::Table,
            Expr::col((ArticleFavorites::Table, ArticleFavorites::ArticleId))
                .eq(Expr::col((Articles::Table, Articles::Id))),
        );

    where_statement(&mut query);

    query
        .group_by_col((Articles::Table, Articles::Id))
        .group_by_col((Users::Table, Users::Id));

    query
}

fn article_list_where_statement(params: &ListArticlesParams, query: &mut SelectStatement) {
    if let Some(tag) = &params.tag {
        query.and_where(Expr::exists(
            Query::select()
                .column((ArticleTags::Table, ArticleTags::ArticleId))
                .from(ArticleTags::Table)
                .inner_join(
                    Tags::Table,
                    Expr::col((ArticleTags::Table, ArticleTags::TagId))
                        .eq(Expr::col((Tags::Table, Tags::Id))),
                )
                .and_where(
                    Expr::col((ArticleTags::Table, ArticleTags::ArticleId))
                        .eq(Expr::col((Articles::Table, Articles::Id)))
                        .and(Expr::col((Tags::Table, Tags::Name)).eq(tag)),
                )
                .to_owned(),
        ));
    }

    if let Some(author_username) = &params.author {
        query.and_where(Expr::col((Users::Table, Users::Username)).eq(author_username.clone()));
    }

    if let Some(favorited_by_username) = &params.favorited_by {
        let favorited_subquery = favorited_subquery(favorited_by_username.clone());
        query.and_where(Expr::col((Articles::Table, Articles::Id)).in_subquery(favorited_subquery));
    }
}

impl ArticleRepository {
    pub fn new(database: Database) -> Self {
        ArticleRepository { database }
    }

    pub async fn insert_article(&self, params: InsertArticleParams) -> Result<Article, AppError> {
        let (sql, values) = Query::insert()
            .into_table(Articles::Table)
            .columns([
                Articles::Slug,
                Articles::Title,
                Articles::Description,
                Articles::Body,
                Articles::AuthorId,
            ])
            .values_panic([
                params.slug.into(),
                params.title.into(),
                params.description.into(),
                params.body.into(),
                params.author_id.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row = sqlx::query_with(&sql, values)
            .fetch_one(self.database.pool())
            .await?;

        Ok(Article::from_row(row))
    }

    pub async fn get_article_by<T>(
        &self,
        field: IndexedArticleField,
        value: T,
    ) -> Result<Option<Article>, AppError>
    where
        sea_query::Value: From<T>,
    {
        let field_name = field.to_field_name();

        let (sql, values) = Query::select()
            .column(Articles::Id)
            .column(Articles::Slug)
            .column(Articles::Title)
            .column(Articles::Description)
            .column(Articles::Body)
            .column(Articles::AuthorId)
            .column(Articles::CreatedAt)
            .column(Articles::UpdatedAt)
            .from(Articles::Table)
            .and_where(Expr::col(field_name).eq(value))
            .build_sqlx(PostgresQueryBuilder);

        let row = sqlx::query_with(&sql, values)
            .fetch_optional(self.database.pool())
            .await?;

        Ok(row.map(Article::from_row))
    }

    pub async fn get_article_view_by<T>(
        &self,
        field: IndexedArticleField,
        value: T,
        user_id: Option<UserId>,
    ) -> Result<Option<ArticleView>, AppError>
    where
        sea_query::Value: From<T>,
        T: Copy,
    {
        let query = build_article_view_query(user_id, move |q| {
            q.and_where(Expr::col(field.to_field_name()).eq(value));
        });

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let row = sqlx::query_with(&sql, values)
            .fetch_optional(self.database.pool())
            .await?;

        Ok(row.map(ArticleView::from_row))
    }

    pub async fn get_article_by_id(
        &self,
        article_id: ArticleId,
        user_id: Option<UserId>,
    ) -> Result<ArticleView, AppError> {
        let query = build_article_view_query(user_id, move |q| {
            q.and_where(Expr::col((Articles::Table, Articles::Id)).eq(article_id));
        });

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let row = sqlx::query_with(&sql, values)
            .fetch_one(self.database.pool())
            .await?;

        Ok(ArticleView::from_row(row))
    }

    pub async fn update_article(&self, params: UpdateArticleParams) -> Result<Article, AppError> {
        let updates = params.as_list();

        if updates.is_empty() {
            return Err(AppError::BadData("No fields to update".to_string()));
        }

        let mut query = Query::update();
        query
            .table(Articles::Table)
            .value(Articles::UpdatedAt, Expr::current_timestamp());

        for (column, value) in &updates {
            query.value(column.clone(), value.clone());
        }

        let (sql, values) = query
            .and_where(Expr::col(Articles::Id).eq(params.article_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row = sqlx::query_with(&sql, values)
            .fetch_one(self.database.pool())
            .await?;

        Ok(Article::from_row(row))
    }

    pub async fn delete_article(&self, article_id: ArticleId) -> Result<(), AppError> {
        let (sql, values) = Query::delete()
            .from_table(Articles::Table)
            .and_where(Expr::col(Articles::Id).eq(article_id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(self.database.pool())
            .await?;

        Ok(())
    }

    pub async fn list_articles(
        &self,
        params: ListArticlesParams,
    ) -> Result<Vec<ArticleListView>, AppError> {
        let mut query =
            build_article_view_query(params.user_id, |q| article_list_where_statement(&params, q));

        let (sql, values) = query
            .order_by((Articles::Table, Articles::CreatedAt), Order::Desc)
            .limit(params.limit.unwrap_or_default().value())
            .offset(params.offset.unwrap_or_default().value())
            .build_sqlx(PostgresQueryBuilder);

        let rows = sqlx::query_with(&sql, values)
            .fetch_all(self.database.pool())
            .await?;

        Ok(rows.into_iter().map(ArticleListView::from_row).collect())
    }

    pub async fn count_articles(&self, params: ListArticlesParams) -> Result<u64, AppError> {
        let subquery =
            build_article_view_query(params.user_id, |q| article_list_where_statement(&params, q));
        let mut query = Query::select();
        query.expr_as(Expr::cust("COUNT(*)"), "count");
        query.from_subquery(subquery, "a");

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let count: i64 = sqlx::query_with(&sql, values)
            .fetch_one(self.database.pool())
            .await?
            .get("count");

        Ok(count as u64)
    }

    pub async fn get_feed_articles(
        &self,
        user_id: UserId,
        limit: Option<Limit>,
        offset: Option<Offset>,
    ) -> Result<Vec<ArticleListView>, AppError> {
        let mut query = build_article_view_query(Some(user_id), |q| {
            q.and_where(Expr::exists(following_subquery(user_id)));
        });

        let (sql, values) = query
            .order_by((Articles::Table, Articles::CreatedAt), Order::Desc)
            .limit(limit.unwrap_or_default().value())
            .offset(offset.unwrap_or_default().value())
            .build_sqlx(PostgresQueryBuilder);

        let rows = sqlx::query_with(&sql, values)
            .fetch_all(self.database.pool())
            .await?;

        Ok(rows.into_iter().map(ArticleListView::from_row).collect())
    }

    pub async fn count_feed_articles(&self, user_id: UserId) -> Result<u64, AppError> {
        let subquery = build_article_view_query(Some(user_id), |q| {
            q.and_where(Expr::exists(following_subquery(user_id)));
        });

        let (sql, values) = Query::select()
            .expr_as(Expr::cust("COUNT(*)"), "count")
            .from_subquery(subquery, "a")
            .build_sqlx(PostgresQueryBuilder);

        let count: i64 = sqlx::query_with(&sql, values)
            .fetch_one(self.database.pool())
            .await?
            .get("count");

        Ok(count as u64)
    }

    pub async fn favorite_article(
        &self,
        user_id: UserId,
        article_id: ArticleId,
    ) -> Result<(), AppError> {
        let (sql, values) = Query::insert()
            .into_table(ArticleFavorites::Table)
            .columns([ArticleFavorites::UserId, ArticleFavorites::ArticleId])
            .values_panic([user_id.into(), article_id.into()])
            .on_conflict(
                sea_query::OnConflict::columns([
                    ArticleFavorites::UserId,
                    ArticleFavorites::ArticleId,
                ])
                .do_nothing()
                .to_owned(),
            )
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(self.database.pool())
            .await?;

        Ok(())
    }

    pub async fn unfavorite_article(
        &self,
        user_id: UserId,
        article_id: ArticleId,
    ) -> Result<(), AppError> {
        let (sql, values) = Query::delete()
            .from_table(ArticleFavorites::Table)
            .and_where(Expr::col(ArticleFavorites::UserId).eq(user_id))
            .and_where(Expr::col(ArticleFavorites::ArticleId).eq(article_id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(self.database.pool())
            .await?;

        Ok(())
    }

    pub async fn add_tags_to_article(
        &self,
        article_id: ArticleId,
        tag_ids: &[uuid::Uuid],
    ) -> Result<(), AppError> {
        for tag_id in tag_ids {
            let (sql, values) = Query::insert()
                .into_table(ArticleTags::Table)
                .columns([ArticleTags::ArticleId, ArticleTags::TagId])
                .values_panic([article_id.into(), (*tag_id).into()])
                .on_conflict(
                    sea_query::OnConflict::columns([ArticleTags::ArticleId, ArticleTags::TagId])
                        .do_nothing()
                        .to_owned(),
                )
                .build_sqlx(PostgresQueryBuilder);

            sqlx::query_with(&sql, values)
                .execute(self.database.pool())
                .await?;
        }

        Ok(())
    }
}
