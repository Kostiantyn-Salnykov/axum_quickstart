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

    pub redis_port: u16,
    pub redis_insight_port: u16,

    pub log_level: String,
}

impl Settings {
    pub fn load() -> Result<Self, envy::Error> {
        dotenvy::dotenv().expect("`.env` not found!");
        envy::from_env::<Self>()
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
}
