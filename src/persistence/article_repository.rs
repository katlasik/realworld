use crate::app_error::AppError;
use crate::database::Database;
use crate::model::persistence::article::Article;
use crate::model::values::article_id::ArticleId;
use crate::model::values::user_id::UserId;
use crate::persistence::params::insert_article_params::InsertArticleParams;
use crate::persistence::params::update_article_params::UpdateArticleParams;
use anyhow::Result;
use sqlx::{Postgres, QueryBuilder, Row};
use tracing::error;
use crate::model::indexed_article_field::IndexedArticleField;
use crate::model::limit::Limit;
use crate::model::offset::Offset;
use crate::model::persistence::article_view::{ArticleListView, ArticleView};
use crate::persistence::params::list_articles_params::ListArticlesParams;

#[derive(Clone)]
pub struct ArticleRepository {
    database: Database,
}

fn article_view_cte<'a>(user_id: Option<UserId>) -> QueryBuilder<'a, Postgres> {
  let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
    r"
        WITH article_view AS (
          SELECT
            a.id,
            a.slug,
            a.title,
            a.description,
            a.created_at,
            a.updated_at,
            a.body,
            u.id as author_id,
            u.username as author_username,
            u.bio as author_bio,
            u.image as author_image,
            COUNT(af.article_id) as favorites_count,
            COALESCE(ARRAY_AGG(t.name) FILTER (WHERE t.name IS NOT NULL), ARRAY[]::text[])::text[] as tag_list
        "
  );

  if let Some(user_id) = user_id {
    query_builder.push(",
            EXISTS(
                SELECT 1 FROM user_follows uf
                WHERE uf.follower_id = ");
    query_builder.push_bind(user_id);
    query_builder.push("
                  AND uf.followee_id = u.id
            ) as following,
            EXISTS(
                SELECT 1 FROM article_favorites af
                WHERE af.user_id = ");
    query_builder.push_bind(user_id);

    query_builder.push(" AND af.article_id = a.id
            ) as favorited ");
  } else {
    query_builder.push(", FALSE as following, FALSE as favorited ");
  }

  query_builder.push(
    r"
            FROM articles a
              INNER JOIN users u ON a.author_id = u.id
              LEFT JOIN article_tags at ON a.id = at.article_id
              LEFT JOIN tags t ON at.tag_id = t.id
              LEFT JOIN article_favorites af ON af.article_id = a.id
            GROUP BY a.id, u.id
          )
        "
  );

  query_builder
}

impl ArticleRepository {
    pub fn new(database: Database) -> Self {
        ArticleRepository { database }
    }

    pub async fn insert_article(&self, params: InsertArticleParams) -> Result<Article, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO articles (slug, title, description, body, author_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, slug, title, description, body, author_id, created_at, updated_at
            "#,
        )
        .bind(&params.slug)
        .bind(&params.title)
        .bind(&params.description)
        .bind(&params.body)
        .bind(params.author_id)
        .fetch_one(self.database.pool())
        .await?;

        Ok(Article::from_row(row))
    }

  pub async fn get_article_by<T>(&self, field: IndexedArticleField, value: &T) -> Result<Option<Article>, AppError>
  where T: for<'a> sqlx::Encode<'a, Postgres> + sqlx::Type<Postgres> + Send
  {

    let sql = format!("SELECT {} FROM articles WHERE {} = $1", Article::column_names("articles").join(", "), field.to_field_name());

    let query = sqlx::query(&sql);

    let row = query.bind(value)
      .fetch_optional(self.database.pool())
      .await?;

    Ok(row.map(Article::from_row))
  }

    pub async fn get_article_view_by<T>(&self, field: IndexedArticleField, value: &T, user_id: Option<UserId>) -> Result<Option<ArticleView>, AppError>
     where T: for<'a> sqlx::Encode<'a, Postgres> + sqlx::Type<Postgres> + Send
    {

        let mut query = article_view_cte(user_id);

      query.push(
          format!("SELECT {} FROM article_view WHERE {} = ", ArticleView::column_names("article_view").join(", "), field.to_field_name()),
        );

      query.push_bind(value);

        let mut query = sqlx::query(query.sql());

       if user_id.is_some() {
         query = query.bind(user_id).bind(user_id);
       }

       let row = query.bind(value)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(row.map(ArticleView::from_row))
    }

    pub async fn get_article_by_id(
        &self,
        article_id: ArticleId,
        user_id: Option<UserId>,
    ) -> Result<ArticleView, AppError> {
      let mut query = article_view_cte(user_id);

      query.push(
        format!("SELECT {} FROM article_view WHERE id = ", ArticleView::column_names("article_view").join(", ")),
      );

      query.push_bind(article_id);

      let row = query.build()
        .fetch_one(self.database.pool())
        .await?;

      Ok(ArticleView::from_row(row))
    }

    pub async fn update_article(&self, params: UpdateArticleParams) -> Result<Article, AppError> {
        let updates = params.as_list();

        if updates.is_empty() {
            return Err(AppError::BadData("No fields to update".to_string()));
        }

        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("UPDATE articles SET updated_at = NOW(), ");

        let mut separated = query_builder.separated(", ");
        let mut idx = 1;
        for (field, _) in &updates {
            separated.push(format!("{} = ${}", field, idx));
            idx += 1;
        }
        query_builder.push(format!(
            " WHERE id = ${} RETURNING id, slug, title, description, body, author_id, created_at, updated_at", idx
        ));

        let mut sql_query = query_builder.build();

        for (_, value) in &updates {
            sql_query = sql_query.bind(value);
        }
        sql_query = sql_query.bind(params.article_id);

        let row = sql_query.fetch_one(self.database.pool()).await?;

        Ok(Article::from_row(row))
    }

    pub async fn delete_article(&self, article_id: ArticleId) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM articles
            WHERE id = $1
            "#,
        )
        .bind(article_id)
        .execute(self.database.pool())
        .await?;

        Ok(())
    }

    pub async fn list_articles(
        &self,
        params: ListArticlesParams
    ) -> Result<Vec<ArticleListView>, AppError> {

      let mut query = article_view_cte(params.user_id);

      query.push(
          r"SELECT * FROM article_view av",
      );

      if params.non_empty() {

        query.push(
          r" WHERE "
        );

        if let Some(tag) = &params.tag {
          query.push_bind(tag);
          query.push(" = ANY(av.tag_list) AND ");

        };

        if let Some(author_username) = &params.author {
          query.push(" av.author_username = ");
          query.push_bind(author_username);
          query.push(" AND ");
        }

        if let Some(favorited_by_username) = &params.favorited_by {
          query.push(
            r#" av.id IN (
                SELECT af.article_id
                FROM article_favorites af
                INNER JOIN users u ON af.user_id = u.id
                WHERE u.username =
              ) "#
          );
          query.push_bind(favorited_by_username.to_string());
        }

        query.push( " 1=1 ");
      }

        query.push(" ORDER BY av.created_at DESC LIMIT ");
        query.push_bind(params.limit.unwrap_or_default().value());
        query.push(" OFFSET ");
        query.push_bind(params.offset.unwrap_or_default().value());

        println!("Generated SQL: {}", query.sql());

        let rows = query
            .build()
            .fetch_all(self.database.pool())
            .await?;

        Ok(rows.into_iter().map(ArticleListView::from_row).collect())
    }

    pub async fn count_articles(
        &self,
        params: ListArticlesParams
    ) -> Result<i64, AppError> {
        let mut query = article_view_cte(params.user_id);

        query.push(
            r"SELECT COUNT(*) FROM article_view av",
        );

        if params.non_empty() {
            query.push(
                r" WHERE "
            );

            if let Some(tag) = &params.tag {
                query.push_bind(tag);
                query.push(" = ANY(av.tag_list) AND ");
            };

            if let Some(author_username) = &params.author {
                query.push(" av.author_username = ");
                query.push_bind(author_username);
                query.push(" AND ");
            }

            if let Some(favorited_by_username) = &params.favorited_by {
                query.push(
                    r#" av.id IN (
                        SELECT af.article_id
                        FROM article_favorites af
                        INNER JOIN users u ON af.user_id = u.id
                        WHERE u.username =
                      ) "#
                );
                query.push_bind(favorited_by_username.to_string());
            }

            query.push( " 1=1 ");
        }

        let row = query
            .build()
            .fetch_one(self.database.pool())
            .await?;

        let count: i64 = row.try_get(0)?;
        Ok(count)
    }

    pub async fn get_feed_articles(
        &self,
        user_id: UserId,
        limit: Option<Limit>,
        offset: Option<Offset>,
    ) -> Result<Vec<ArticleListView>, AppError> {

        let mut sql = article_view_cte(Some(user_id));

      sql.push(
            format!(r#"
            SELECT {}
            FROM article_view av
            INNER JOIN user_follows uf ON av.author_id = uf.followee_id
            WHERE uf.follower_id = $1
            ORDER BY av.created_at DESC
            LIMIT
            "#, ArticleListView::column_names("av").join(", "))
        );
        sql.push_bind(limit.unwrap_or_default().value());
        sql.push(" OFFSET ");
        sql.push_bind(offset.unwrap_or_default().value());

        error!("Generated SQL: {}", sql.sql());

        let query = sqlx::query(sql.sql())
          .bind(user_id)
          .bind(user_id)
          .bind(limit.unwrap_or_default().value())
          .bind(offset.unwrap_or_default().value());

          let rows = query.fetch_all(self.database.pool()).await?;

        Ok(rows.into_iter().map(ArticleListView::from_row).collect())
    }

    pub async fn favorite_article(
        &self,
        user_id: UserId,
        article_id: ArticleId,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO article_favorites (user_id, article_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id, article_id) DO NOTHING
            "#,
        )
        .bind(user_id)
        .bind(article_id)
        .execute(self.database.pool())
        .await?;

        Ok(())
    }

    pub async fn unfavorite_article(
        &self,
        user_id: UserId,
        article_id: ArticleId,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM article_favorites
            WHERE user_id = $1 AND article_id = $2
            "#,
        )
        .bind(user_id)
        .bind(article_id)
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
            sqlx::query(
                r#"
                INSERT INTO article_tags (article_id, tag_id)
                VALUES ($1, $2)
                ON CONFLICT (article_id, tag_id) DO NOTHING
                "#,
            )
            .bind(article_id)
            .bind(tag_id)
            .execute(self.database.pool())
            .await?;
        }

        Ok(())
    }

}
