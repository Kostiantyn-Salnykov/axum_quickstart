use redis::{
    Client, RedisResult,
    aio::{ConnectionManager, ConnectionManagerConfig},
};

#[derive(Clone)]
pub struct RedisClient {
    manager: ConnectionManager,
    url: String,
}

impl RedisClient {
    pub fn new(url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(url)?;
        let manager = client.get_connection_manager_lazy(ConnectionManagerConfig::new())?;

        Ok(Self {
            manager,
            url: url.to_string(),
        })
    }

    pub fn connection(&self) -> RedisResult<ConnectionManager> {
        Ok(self.manager.clone())
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}
