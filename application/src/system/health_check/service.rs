use crate::errors::ServiceError;
use crate::system::health_check::port::HealthCheckPort;
use crate::system::health_check::result::HealthCheckResult;
use crate::system::health_check::use_case::HealthCheckUseCase;
use std::sync::Arc;

#[derive(Clone)]
pub struct HealthCheckService {
    provider: Arc<dyn HealthCheckPort>,
}

impl HealthCheckService {
    pub fn new(provider: Arc<dyn HealthCheckPort>) -> Self {
        Self { provider }
    }
}

#[async_trait::async_trait]
impl HealthCheckUseCase for HealthCheckService {
    async fn check(&self) -> Result<HealthCheckResult, ServiceError> {
        self.provider.check().await
    }
}
