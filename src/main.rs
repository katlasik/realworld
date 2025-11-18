use ::tracing::info;
use axum::Router;
use crate::app_config::load_config;
use crate::database::connect_db;
use crate::http::router;
use crate::server::init_server;
use crate::tracing::init_tracing;

mod app_config;
mod http;
mod database;
mod tracing;
mod server;
mod model;

#[tokio::main]
async fn main() {

    init_tracing();
    info!("Starting realworld server...");
    let config = load_config();
    let db = connect_db(&config.database).await.unwrap();
    init_server(&config.http).await.unwrap();

}
