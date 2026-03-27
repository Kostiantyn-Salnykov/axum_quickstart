use std::sync::Arc;

use application::system::health_check::inbound::HealthCheck;
use application::users::register::inbound::RegisterUser;

#[derive(Clone)]
pub struct AppState {
    pub health_check: Arc<dyn HealthCheck>,
    pub register_user: Arc<dyn RegisterUser>,
}
