use crate::errors::ServiceError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait TokenBlacklistPort: Send + Sync {
    async fn contains(&self, token: &str) -> Result<bool, ServiceError>;
    async fn revoke_until(
        &self,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), ServiceError>;
}
