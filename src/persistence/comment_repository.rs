use crate::app_error::AppError;
use crate::database::Database;
use crate::model::persistence::comment::Comment;
use crate::model::persistence::comment_view::CommentView;
use crate::model::values::article_id::ArticleId;
use crate::model::values::comment_id::CommentId;
use crate::model::values::user_id::UserId;
use crate::persistence::params::insert_comment_params::InsertCommentParams;
use crate::persistence::schema::{Comments, UserFollows, Users};
use anyhow::Result;
use sea_query::{Alias, Expr, Order, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::Row;

#[derive(Clone)]
pub struct CommentRepository {
    database: Database,
}

fn comment_view_query(user_id: Option<UserId>) -> sea_query::SelectStatement {

  let mut select = Query::select();

  select
    .column((Comments::Table, Comments::Id))
    .column((Comments::Table, Comments::Body))
    .column((Comments::Table, Comments::CreatedAt))
    .column((Comments::Table, Comments::UpdatedAt))
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
    .from(Comments::Table)
    .inner_join(
      Users::Table,
      Expr::col((Comments::Table, Comments::AuthorId))
        .eq(Expr::col((Users::Table, Users::Id))),
    );

  match user_id {
    Some(user_id) => {
      let subquery = Query::select()
        .expr(Expr::val(1))
        .from(UserFollows::Table)
        .and_where(
          Expr::col((UserFollows::Table, UserFollows::FollowerId)).eq(user_id)
            .and(
              Expr::col((UserFollows::Table, UserFollows::FolloweeId))
                .eq(Expr::col((Users::Table, Users::Id))),
            ),
        )
        .to_owned();

      select.expr_as(Expr::exists(subquery), Alias::new("following"));
    }
    None => {
      select.expr_as(Expr::val(false), Alias::new("following"));
    }
  }

  select
}

impl CommentRepository {
    pub fn new(database: Database) -> Self {
        CommentRepository { database }
    }

    pub async fn insert_comment(&self, params: InsertCommentParams) -> Result<Comment, AppError> {
        let (sql, values) = Query::insert()
            .into_table(Comments::Table)
            .columns([Comments::Body, Comments::ArticleId, Comments::AuthorId])
            .values_panic([
                params.body.into(),
                params.article_id.into(),
                params.author_id.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row = sqlx::query_with(&sql, values)
            .fetch_one(self.database.pool())
            .await?;

        Ok(Comment::from_row(row))
    }

    pub async fn delete_comment(&self, comment_id: CommentId) -> Result<(), AppError> {
        let (sql, values) = Query::delete()
            .from_table(Comments::Table)
            .and_where(Expr::col(Comments::Id).eq(comment_id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(self.database.pool())
            .await?;

        Ok(())
    }

    pub async fn is_comment_author(
        &self,
        comment_id: CommentId,
        user_id: UserId,
    ) -> Result<bool, AppError> {
        let subquery = Query::select()
            .expr(Expr::value(1))
            .from(Comments::Table)
            .and_where(Expr::col(Comments::Id).eq(comment_id))
            .and_where(Expr::col(Comments::AuthorId).eq(user_id))
            .to_owned();

        let (sql, values) = Query::select()
            .expr_as(Expr::exists(subquery), "is_author")
            .build_sqlx(PostgresQueryBuilder);

        let row = sqlx::query_with(&sql, values)
            .fetch_one(self.database.pool())
            .await?;

        Ok(row.get("is_author"))
    }

    pub async fn get_comments(
        &self,
        article_id: ArticleId,
        user_id: Option<UserId>,
    ) -> Result<Vec<CommentView>, AppError> {
        let mut query = comment_view_query(user_id);

      let (sql, values) = query.and_where(Expr::col(Comments::ArticleId).eq(article_id))
          .order_by(Comments::CreatedAt, Order::Desc)
        .build_sqlx(PostgresQueryBuilder);


      let rows = sqlx::query_with(&sql, values)
        .fetch_all(self.database.pool())
        .await?;

        Ok(rows.into_iter().map(CommentView::from_row).collect())
    }

    pub async fn get_comment(
        &self,
        comment_id: CommentId,
        user_id: Option<UserId>,
    ) -> Result<CommentView, AppError> {
      let mut query = comment_view_query(user_id);

      let (sql, values) = query.and_where(Expr::col((Comments::Table, Comments::Id)).eq(comment_id))
        .order_by(Comments::CreatedAt, Order::Desc)
        .build_sqlx(PostgresQueryBuilder);


      let row = sqlx::query_with(&sql, values)
        .fetch_one(self.database.pool())
        .await?;

      Ok(CommentView::from_row(row))
    }
}
