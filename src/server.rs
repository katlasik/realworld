use std::io::Error;
use axum::Router;
use tracing::info;
use crate::app_config::HttpConfig;
use crate::http::router;

pub async fn init_server(config: &HttpConfig) -> Result<(), Error> {
  let listener = tokio::net::TcpListener::bind(config.url())
    .await?;

  let router = Router::new()
    .nest("/api", router());

  axum::serve(listener, router)
    .await?;

  info!("Server running on {}", config.url());
  
  Ok(())
}