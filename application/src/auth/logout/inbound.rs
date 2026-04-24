use crate::errors::ServiceError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait LogoutUseCase: Send + Sync {
    async fn logout(&self, token: String, expires_at: DateTime<Utc>) -> Result<(), ServiceError>;
}
