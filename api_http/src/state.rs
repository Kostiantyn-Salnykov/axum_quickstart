use application::auth::login::inbound::Login;
use application::auth::logout::inbound::Logout;
use application::auth::refresh::inbound::Refresh;
use application::auth::register::inbound::Register;
use application::auth::token_manager::TokenManager;
use application::system::health_check::inbound::HealthCheck;
use application::users::get::inbound::GetUser;
use std::sync::Arc;

#[derive(Clone)]
pub struct SystemState {
    pub health_check: Arc<dyn HealthCheck>,
}

#[derive(Clone)]
pub struct AuthState {
    pub register: Arc<dyn Register>,
    pub login: Arc<dyn Login>,
    pub logout: Arc<dyn Logout>,
    pub refresh: Arc<dyn Refresh>,
    pub token_manager: Arc<dyn TokenManager>,
}

#[derive(Clone)]
pub struct UsersState {
    pub get: Arc<dyn GetUser>,
}

#[derive(Clone)]
pub struct AppState {
    pub system: SystemState,
    pub auth: AuthState,
    pub users: UsersState,
}
