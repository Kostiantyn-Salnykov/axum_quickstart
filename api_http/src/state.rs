use application::auth::login::inbound::LoginUseCase;
use application::auth::logout::inbound::LogoutUseCase;
use application::auth::refresh::inbound::RefreshUseCase;
use application::auth::register::inbound::RegisterUseCase;
use application::auth::token_manager::TokenManager;
use application::system::health_check::inbound::HealthCheckUseCase;
use application::users::get::inbound::GetUserUseCase;
use std::sync::Arc;

#[derive(Clone)]
pub struct SystemState {
    pub health_check: Arc<dyn HealthCheckUseCase>,
}

#[derive(Clone)]
pub struct AuthState {
    pub register: Arc<dyn RegisterUseCase>,
    pub login: Arc<dyn LoginUseCase>,
    pub logout: Arc<dyn LogoutUseCase>,
    pub refresh: Arc<dyn RefreshUseCase>,
    pub token_manager: Arc<dyn TokenManager>,
}

#[derive(Clone)]
pub struct UsersState {
    pub get: Arc<dyn GetUserUseCase>,
}

#[derive(Clone)]
pub struct AppState {
    pub system: SystemState,
    pub auth: AuthState,
    pub users: UsersState,
}
