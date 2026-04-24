use crate::errors::ServiceError;
use crate::system::health_check::inbound::HealthCheckUseCase;
use crate::system::health_check::outbound::HealthCheckPort;
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
    async fn check(&self) -> Result<String, ServiceError> {
        self.provider.check().await
    }
}
