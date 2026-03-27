use crate::errors::ServiceError;

#[async_trait::async_trait]
pub trait DatabaseHealthCheck: Send + Sync {
    async fn current_timestamp(&self) -> Result<String, ServiceError>;
}
