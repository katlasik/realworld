use crate::app_error::AppError;
use crate::domain::commands::add_comment_command::AddCommentCommand;
use crate::http::AppState;
use crate::http::dto::comment::{CommentItem, CommentResponse, CommentsResponse, CreateCommentRequest};
use crate::http::extractors::auth_token::AuthToken;
use crate::model::values::comment_id::CommentId;
use crate::model::values::slug::Slug;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use tracing::info;

pub(crate) fn comment_routes() -> Router<AppState> {
    Router::new()
        .route("/articles/{slug}/comments", post(add_comment))
        .route("/articles/{slug}/comments", get(get_comments))
        .route("/articles/{slug}/comments/{id}", delete(delete_comment))
}

async fn add_comment(
    State(state): State<AppState>,
    auth: AuthToken,
    Path(slug): Path<Slug>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<CommentResponse>), AppError> {
    info!("Add comment to article: {}", slug);

    let article = state
        .article_service
        .get_article(&slug, Some(auth.user_id))
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    let command = AddCommentCommand::from_request(payload, article.id, auth.user_id);

    let comment_view = state
        .comment_service
        .add_comment(command, auth.user_id)
        .await?;

    let comment = CommentItem::from_comment_view(comment_view);

    Ok((StatusCode::CREATED, Json(CommentResponse { comment })))
}

async fn get_comments(
    State(state): State<AppState>,
    auth: Option<AuthToken>,
    Path(slug): Path<Slug>,
) -> Result<Json<CommentsResponse>, AppError> {
    info!("Get comments for article: {}", slug);

    let article = state
        .article_service
        .get_article(&slug, None)
        .await?
        .ok_or_else(|| AppError::NotFound)?;

    let comment_views = state
        .comment_service
        .get_comments(article.id, auth.map(|a| a.user_id))
        .await?;

    let comments = comment_views
        .into_iter()
        .map(CommentItem::from_comment_view)
        .collect();

    Ok(Json(CommentsResponse { comments }))
}

async fn delete_comment(
    State(state): State<AppState>,
    auth: AuthToken,
    Path((slug, comment_id)): Path<(Slug, CommentId)>,
) -> Result<StatusCode, AppError> {
    info!("Delete comment {} from article: {}", comment_id, slug);

    state
        .comment_service
        .delete_comment(comment_id, auth.user_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
