use std::sync::Arc;

use service::use_cases::health_check::HealthCheckUseCase;

#[derive(Clone)]
pub struct AppState {
    pub health_check: Arc<dyn HealthCheckUseCase>,
}
