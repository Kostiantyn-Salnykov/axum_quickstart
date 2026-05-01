use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub postgres_db: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_host: String,
    pub postgres_port: u16,

    pub server_port: u16,

    pub pgadmin_listen_port: u16,

    #[serde(default = "default_redis_host")]
    pub redis_host: String,
    pub redis_port: u16,
    pub redis_insight_port: u16,

    pub log_level: String,
    #[serde(default)]
    pub log_settings: Option<String>,
    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
    #[serde(default = "default_access_token_ttl_minutes")]
    pub access_token_ttl_minutes: i64,
    #[serde(default = "default_refresh_token_ttl_days")]
    pub refresh_token_ttl_days: i64,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        dotenvy::dotenv().map_err(|error| ConfigError::Foreign(Box::new(error)))?;

        Config::builder()
            .add_source(
                Environment::default()
                    .convert_case(config::Case::Snake)
                    .try_parsing(true),
            )
            .build()?
            .try_deserialize()
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{username}:{password}@{host}:{port}/{db}",
            username = self.postgres_user,
            password = self.postgres_password,
            host = self.postgres_host,
            port = self.postgres_port,
            db = self.postgres_db,
        )
    }

    pub fn server_addr(&self) -> String {
        format!("0.0.0.0:{port}", port = self.server_port)
    }

    pub fn redis_url(&self) -> String {
        format!(
            "redis://{host}:{port}/",
            host = self.redis_host,
            port = self.redis_port
        )
    }

    pub fn should_log_settings(&self) -> bool {
        self.log_settings
            .as_deref()
            .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }
}

fn default_jwt_secret() -> String {
    "change-me-super-secret-jwt-key".to_string()
}

fn default_access_token_ttl_minutes() -> i64 {
    60
}

fn default_refresh_token_ttl_days() -> i64 {
    7
}

fn default_redis_host() -> String {
    "127.0.0.1".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn load_from(source: HashMap<String, String>) -> Settings {
        Config::builder()
            .add_source(
                Environment::default()
                    .source(Some(source))
                    .convert_case(config::Case::Snake)
                    .try_parsing(true),
            )
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }

    fn base_env() -> HashMap<String, String> {
        HashMap::from([
            ("POSTGRES_DB".to_string(), "postgres".to_string()),
            ("POSTGRES_USER".to_string(), "postgres".to_string()),
            ("POSTGRES_PASSWORD".to_string(), "postgres".to_string()),
            ("POSTGRES_HOST".to_string(), "127.0.0.1".to_string()),
            ("POSTGRES_PORT".to_string(), "5432".to_string()),
            ("SERVER_PORT".to_string(), "9999".to_string()),
            ("PGADMIN_LISTEN_PORT".to_string(), "8080".to_string()),
            ("REDIS_PORT".to_string(), "6379".to_string()),
            ("REDIS_INSIGHT_PORT".to_string(), "8001".to_string()),
            ("LOG_LEVEL".to_string(), "trace,tower_http=info".to_string()),
        ])
    }

    #[test]
    fn loads_settings_from_uppercase_env_style_keys() {
        let settings = load_from(base_env());

        assert_eq!(settings.postgres_db, "postgres");
        assert_eq!(settings.postgres_port, 5432);
        assert_eq!(settings.server_port, 9999);
        assert_eq!(settings.redis_host, "127.0.0.1");
    }

    #[test]
    fn computes_derived_urls() {
        let settings = load_from(base_env());

        assert_eq!(
            settings.database_url(),
            "postgres://postgres:postgres@127.0.0.1:5432/postgres"
        );
        assert_eq!(settings.server_addr(), "0.0.0.0:9999");
        assert_eq!(settings.redis_url(), "redis://127.0.0.1:6379/");
    }

    #[test]
    fn enables_settings_logging_only_for_truthy_values() {
        let mut source = base_env();
        source.insert("LOG_SETTINGS".to_string(), "true".to_string());
        assert!(load_from(source).should_log_settings());

        let mut source = base_env();
        source.insert("LOG_SETTINGS".to_string(), "1".to_string());
        assert!(load_from(source).should_log_settings());

        let mut source = base_env();
        source.insert("LOG_SETTINGS".to_string(), "false".to_string());
        assert!(!load_from(source).should_log_settings());
    }
}
