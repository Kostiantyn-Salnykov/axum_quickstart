use crate::errors::ServiceError;
use crate::ports::health_check::HealthCheckProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct HealthCheckService {
    provider: Arc<dyn HealthCheckProvider>,
}

impl HealthCheckService {
    pub fn new(provider: Arc<dyn HealthCheckProvider>) -> Self {
        Self { provider }
    }

    pub async fn check(&self) -> Result<String, ServiceError> {
        self.provider.current_timestamp().await
    }
}
