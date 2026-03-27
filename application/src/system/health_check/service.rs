use crate::errors::ServiceError;
use crate::system::health_check::inbound::HealthCheck;
use crate::system::health_check::outbound::DatabaseHealthCheck;
use std::sync::Arc;

#[derive(Clone)]
pub struct HealthCheckService {
    provider: Arc<dyn DatabaseHealthCheck>,
}

impl HealthCheckService {
    pub fn new(provider: Arc<dyn DatabaseHealthCheck>) -> Self {
        Self { provider }
    }
}

#[async_trait::async_trait]
impl HealthCheck for HealthCheckService {
    async fn check(&self) -> Result<String, ServiceError> {
        self.provider.current_timestamp().await
    }
}
