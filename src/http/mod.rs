pub(crate) mod dto;
pub(crate) mod extractors;
mod routes;

use routes::*;

use crate::app_config::AppConfig;
use crate::domain::user_service::UserService;
use crate::utils::jwt::JwtGenerator;
use axum::Router;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

pub(crate) fn router() -> Router<AppState> {
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

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub user_service: UserService,
    pub jwt_generator: JwtGenerator,
}
