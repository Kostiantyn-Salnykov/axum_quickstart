use std::sync::Arc;

use application::system::health_check::inbound::HealthCheck;
use application::system::health_check::outbound::DatabaseHealthCheck;
use application::system::health_check::service::HealthCheckService;

pub fn build_health_check_service(provider: Arc<dyn DatabaseHealthCheck>) -> Arc<dyn HealthCheck> {
    Arc::new(HealthCheckService::new(provider))
}
