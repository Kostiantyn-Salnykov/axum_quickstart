use crate::errors::ServiceError;
use crate::ports::health_check::HealthCheckProvider;
use crate::use_cases::health_check::HealthCheckUseCase;
use std::sync::Arc;

#[derive(Clone)]
pub struct HealthCheckService {
    provider: Arc<dyn HealthCheckProvider>,
}

impl HealthCheckService {
    pub fn new(provider: Arc<dyn HealthCheckProvider>) -> Self {
        Self { provider }
    }
}

#[async_trait::async_trait]
impl HealthCheckUseCase for HealthCheckService {
    async fn check(&self) -> Result<String, ServiceError> {
        self.provider.current_timestamp().await
    }
}
