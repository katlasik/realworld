use tryphon::{Config, ErrorPrintMode, Secret};

#[derive(Debug, Config, Clone)]
pub struct HttpConfig {
    #[env("HTTP_HOST")]
    #[default("0.0.0.0")]
    pub(crate) host: String,
    #[env("HTTP_PORT")]
    #[default(8080)]
    pub(crate) port: u16,
}

impl HttpConfig {
    pub(crate) fn url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Config, Clone)]
pub struct DatabaseConfig {
    #[env("DATABASE_USER")]
    pub(crate) user: String,
    #[env("DATABASE_NAME")]
    pub(crate) database: String,
    #[env("DATABASE_PASSWORD")]
    pub(crate) password: Secret<String>,
    #[env("DATABASE_ADDRESS")]
    pub(crate) address: String,
    #[env("DATABASE_MAX_CONNECTIONS")]
    #[default(5)]
    pub(crate) max_connections: u32,
}

impl DatabaseConfig {
    pub(crate) fn connection_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}/{}",
            self.user, *self.password, self.address, self.database
        )
    }
}

#[derive(Debug, Config, Clone)]
pub struct SecretsConfig {
    #[env("PASSWORD_PEPPER")]
    #[default(Secret("default_pepper".to_string()))]
    pub(crate) pepper: Secret<String>,
    #[env("JWT_SECRET")]
    #[default(Secret("default_jwt_secret_change_in_production".to_string()))]
    pub(crate) jwt: Secret<String>,
}

#[derive(Debug, Config, Clone)]
pub struct AppConfig {
    #[config]
    pub(crate) http: HttpConfig,
    #[config]
    pub(crate) database: DatabaseConfig,
    #[config]
    pub(crate) secrets: SecretsConfig,
}

pub fn load_config() -> AppConfig {
    dotenvy::dotenv().ok();

    match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!(
                "Couldn't load configuration from env variables:\n{}",
                e.pretty_print(ErrorPrintMode::Table)
            );
            panic!("Configuration loading failed");
        }
    }
}
