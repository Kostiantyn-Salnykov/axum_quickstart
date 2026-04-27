use application::auth::token_blacklist_port::TokenBlacklistPort;
use application::errors::ServiceError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use redis::AsyncCommands;
use sha2::{Digest, Sha256};

use crate::adapters::cache::redis_client::RedisClient;

#[derive(Clone)]
pub struct RedisTokenBlacklistAdapter {
    client: RedisClient,
}

impl RedisTokenBlacklistAdapter {
    pub fn new(client: RedisClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl TokenBlacklistPort for RedisTokenBlacklistAdapter {
    async fn contains(&self, token: &str) -> Result<bool, ServiceError> {
        let key = blacklist_key(token);
        let mut connection = self.client.connection().map_err(|error| {
            tracing::error!(
                error = ?error,
                redis_url = %self.client.url(),
                operation = "connect",
                "Failed to acquire Redis connection for blacklist lookup."
            );
            ServiceError::internal(error)
        })?;
        let exists: bool = connection.exists(&key).await.map_err(|error| {
            tracing::error!(
                error = ?error,
                redis_url = %self.client.url(),
                operation = "exists",
                key = %key,
                "Failed to query Redis token blacklist."
            );
            ServiceError::internal(error)
        })?;
        Ok(exists)
    }

    async fn revoke_until(
        &self,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), ServiceError> {
        let ttl_seconds = expires_at.signed_duration_since(Utc::now()).num_seconds();
        if ttl_seconds <= 0 {
            return Err(ServiceError::InvalidCredentials);
        }

        let key = blacklist_key(token);
        let mut connection = self.client.connection().map_err(|error| {
            tracing::error!(
                error = ?error,
                redis_url = %self.client.url(),
                operation = "connect",
                "Failed to acquire Redis connection for blacklist write."
            );
            ServiceError::internal(error)
        })?;
        let _: () = connection
            .set_ex(&key, "1", ttl_seconds as u64)
            .await
            .map_err(|error| {
                tracing::error!(
                    error = ?error,
                    redis_url = %self.client.url(),
                    operation = "set_ex",
                    key = %key,
                    ttl_seconds,
                    "Failed to write token into Redis blacklist."
                );
                ServiceError::internal(error)
            })?;

        Ok(())
    }
}

fn blacklist_key(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut hex, "{byte:02x}");
    }

    format!("jwt:blacklist:{hex}")
}
