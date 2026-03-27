use std::sync::Arc;

use application::ports::inbound::health_check::HealthCheck;
use application::ports::inbound::register_user::RegisterUser;

#[derive(Clone)]
pub struct AppState {
    pub health_check: Arc<dyn HealthCheck>,
    pub register_user: Arc<dyn RegisterUser>,
}
