use crate::app_config::DatabaseConfig;
use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};
use tracing::info;

#[derive(Clone)]
pub struct Database(Pool<Postgres>);

impl Database {
    pub fn pool(&self) -> &Pool<Postgres> {
        &self.0
    }
}

pub async fn connect_db(config: &DatabaseConfig) -> Result<Database, Error> {
    info!(
        "Connecting to database {} with user {} successfully. Hash of password: {}.",
        config.connection_url(),
        config.user,
        config.password
    );

    let db = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.connection_string())
        .await?;

    info!(
        "Connected to database {} with user {} successfully. Hash of password: {}",
        config.database, config.user, config.password
    );

    migrate(&db).await?;

    Ok(Database(db))
}

pub async fn migrate(db_pool: &Pool<Postgres>) -> Result<(), MigrateError> {
    tracing::log::info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(db_pool).await
}
