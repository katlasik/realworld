use tracing::error;
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
    #[default("realworld")]
    pub(crate) user: String,
    #[env("DATABASE_NAME")]
    #[default("realworld")]
    pub(crate) database: String,
    #[env("DATABASE_PASSWORD")]
    #[default(Secret("password".into()))]
    pub(crate) password: Secret<String>,
    #[env("DATABASE_HOST")]
    #[default("localhost")]
    pub(crate) host: String,
    #[env("DATABASE_PORT")]
    #[default(5432)]
    pub(crate) port: u16,
    #[env("DATABASE_MAX_CONNECTIONS")]
    #[default(50)]
    pub(crate) max_connections: u32,
}

impl DatabaseConfig {
    pub fn connection_url(&self) -> String {
        format!("{}:{}/{}", self.host, self.port, self.database)
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}",
            self.user,
            *self.password,
            self.connection_url()
        )
    }
}

#[derive(Debug, Config, Clone)]
pub struct SecretsConfig {
    #[env("PASSWORD_PEPPER")]
    #[default(Secret("default_pepper".to_string()))]
    pub pepper: Secret<String>,
    #[env("JWT_SECRET")]
    #[default(Secret("default_jwt_secret_change_in_production".to_string()))]
    pub jwt: Secret<String>,
}

#[derive(Debug, Config, Clone)]
pub struct AppConfig {
    #[config]
    pub http: HttpConfig,
    #[config]
    pub database: DatabaseConfig,
    #[config]
    pub secrets: SecretsConfig,
}

pub fn load_config() -> AppConfig {
    dotenvy::dotenv().ok();

    match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!(
                "Couldn't load configuration from env variables:\n{}",
                e.pretty_print(ErrorPrintMode::Table)
            );
            panic!("Configuration loading failed");
        }
    }
}
