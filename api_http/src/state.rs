use application::auth::login::inbound::Login;
use application::auth::refresh::inbound::Refresh;
use application::auth::register::inbound::Register;
use std::sync::Arc;

use application::system::health_check::inbound::HealthCheck;
use application::users::get::inbound::GetUser;

#[derive(Clone)]
pub struct AppState {
    pub health_check: Arc<dyn HealthCheck>,
    pub auth_register: Arc<dyn Register>,
    pub auth_login: Arc<dyn Login>,
    pub auth_refresh: Arc<dyn Refresh>,
    pub get_user: Arc<dyn GetUser>,
}
