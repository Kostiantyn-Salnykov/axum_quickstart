use std::sync::Arc;

use service::services::health_check::HealthCheckService;

#[derive(Clone)]
pub struct AppState {
    pub health_check_service: Arc<HealthCheckService>,
}
