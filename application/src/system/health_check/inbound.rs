use crate::errors::ServiceError;
use crate::system::health_check::result::HealthCheckResult;

#[async_trait::async_trait]
pub trait HealthCheckUseCase: Send + Sync {
    async fn check(&self) -> Result<HealthCheckResult, ServiceError>;
}
