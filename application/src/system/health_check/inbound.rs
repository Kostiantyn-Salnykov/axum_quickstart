use crate::errors::ServiceError;

#[async_trait::async_trait]
pub trait HealthCheckUseCase: Send + Sync {
    async fn check(&self) -> Result<String, ServiceError>;
}
