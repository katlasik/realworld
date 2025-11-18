mod routes;
mod dto;

use routes::*;

use axum::Router;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

pub(crate) fn router() -> Router {
  Router::new()
    .merge(auth::auth_routes())
    .merge(users::user_routes())
    .merge(profiles::profile_routes())
    .merge(articles::article_routes())
    .merge(comments::comment_routes())
    .merge(tags::tag_routes())
    .merge(health::health_routes())
    .layer(
      TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
    )
}
