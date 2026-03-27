use crate::errors::ServiceError;

#[async_trait::async_trait]
pub trait HealthCheck: Send + Sync {
    async fn check(&self) -> Result<String, ServiceError>;
}
