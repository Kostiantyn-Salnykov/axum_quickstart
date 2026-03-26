use std::sync::Arc;

use service::use_cases::health_check::HealthCheckUseCase;
use service::use_cases::register_user::RegisterUserUseCase;

#[derive(Clone)]
pub struct AppState {
    pub health_check: Arc<dyn HealthCheckUseCase>,
    pub register_user: Arc<dyn RegisterUserUseCase>,
}
