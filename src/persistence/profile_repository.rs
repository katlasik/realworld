use crate::app_error::AppError;
use crate::database::Database;
use crate::model::values::user_id::UserId;
use crate::persistence::schema::UserFollows;
use anyhow::Result;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::Row;

#[derive(Clone)]
pub struct ProfileRepository {
    database: Database,
}

impl ProfileRepository {
    pub fn new(database: Database) -> Self {
        ProfileRepository { database }
    }

    pub async fn follow_user(
        &self,
        follower_id: UserId,
        followee_id: UserId,
    ) -> Result<(), AppError> {
        let (sql, values) = Query::insert()
            .into_table(UserFollows::Table)
            .columns([UserFollows::FollowerId, UserFollows::FolloweeId])
            .values_panic([follower_id.into(), followee_id.into()])
            .on_conflict(
                sea_query::OnConflict::columns([
                    UserFollows::FollowerId,
                    UserFollows::FolloweeId,
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

    pub async fn unfollow_user(
        &self,
        follower_id: UserId,
        followee_id: UserId,
    ) -> Result<(), AppError> {
        let (sql, values) = Query::delete()
            .from_table(UserFollows::Table)
            .and_where(Expr::col(UserFollows::FollowerId).eq(follower_id))
            .and_where(Expr::col(UserFollows::FolloweeId).eq(followee_id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(self.database.pool())
            .await?;

        Ok(())
    }

    pub async fn is_following(
        &self,
        follower_id: UserId,
        followee_id: UserId,
    ) -> Result<bool, AppError> {
        let subquery = Query::select()
            .expr(Expr::val(1))
            .from(UserFollows::Table)
            .and_where(Expr::col(UserFollows::FollowerId).eq(follower_id))
            .and_where(Expr::col(UserFollows::FolloweeId).eq(followee_id))
            .to_owned();

        let (sql, values) = Query::select()
            .expr_as(Expr::exists(subquery), sea_query::Alias::new("is_following"))
            .build_sqlx(PostgresQueryBuilder);

        let row = sqlx::query_with(&sql, values)
            .fetch_one(self.database.pool())
            .await?;

        Ok(row.get("is_following"))
    }
}
