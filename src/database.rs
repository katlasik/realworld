use sqlx::{pool, Error, Pool, Postgres};
use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use crate::app_config::DatabaseConfig;

pub struct Database(Pool<Postgres>);

pub async fn connect_db(config: &DatabaseConfig) -> Result<Database, Error> {

  info!("Connecting to database...");

  let db = PgPoolOptions::new()
    .max_connections(config.max_connections)
    .connect(&config.connection_url())
    .await?;

  info!("Connected to database {} with user {} successfully. Hash of password: {}", config.database, config.user, config.password);

  migrate(&db).await?;

  Ok(Database(db))
}

async fn migrate(db_pool: &Pool<Postgres>) -> Result<(), MigrateError> {
  tracing::log::info!("Running database migrations...");
  sqlx::migrate!("./migrations")
    .run(db_pool)
    .await
}