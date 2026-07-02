use application::errors::ServiceError;
use redis::AsyncCommands;

use crate::adapters::redis::client::RedisClient;

#[derive(Clone)]
pub struct RedisHealthCheck {
    client: RedisClient,
}

impl RedisHealthCheck {
    pub fn new(client: RedisClient) -> Self {
        Self { client }
    }

    pub async fn check(&self) -> Result<String, ServiceError> {
        let mut connection = self.client.connection().map_err(|error| {
            tracing::error!(
                error = ?error,
                redis_url = %self.client.url(),
                operation = "connect",
                "Failed to acquire Redis connection for health check."
            );
            ServiceError::internal(error)
        })?;

        let pong: String = connection.ping().await.map_err(|error| {
            tracing::error!(
                error = ?error,
                redis_url = %self.client.url(),
                operation = "ping",
                "Failed to execute Redis health check."
            );
            ServiceError::internal(error)
        })?;

        Ok(pong)
    }
}
