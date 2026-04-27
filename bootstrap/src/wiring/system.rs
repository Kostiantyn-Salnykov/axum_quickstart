use std::sync::Arc;

use application::system::health_check::port::HealthCheckPort;
use application::system::health_check::service::HealthCheckService;
use application::system::health_check::use_case::HealthCheckUseCase;

pub fn build_health_check_service(
    provider: Arc<dyn HealthCheckPort>,
) -> Arc<dyn HealthCheckUseCase> {
    Arc::new(HealthCheckService::new(provider))
}
