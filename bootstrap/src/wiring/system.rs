use std::sync::Arc;

use application::system::health_check::inbound::HealthCheckUseCase;
use application::system::health_check::outbound::HealthCheckPort;
use application::system::health_check::service::HealthCheckService;

pub fn build_health_check_service(
    provider: Arc<dyn HealthCheckPort>,
) -> Arc<dyn HealthCheckUseCase> {
    Arc::new(HealthCheckService::new(provider))
}
