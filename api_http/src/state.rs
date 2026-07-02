use application::auth::login::use_case::LoginUseCase;
use application::auth::logout::use_case::LogoutUseCase;
use application::auth::refresh::use_case::RefreshUseCase;
use application::auth::register::use_case::RegisterUseCase;
use application::auth::verify_access_token::use_case::VerifyAccessTokenUseCase;
use application::rate_limit::rate_limiter_port::RateLimiterPort;
use application::search::use_case::SearchUseCase;
use application::system::health_check::use_case::HealthCheckUseCase;
use application::users::get::use_case::GetUserUseCase;
use application::users::search::query::UserSearchField;
use application::users::search::result::UserSearchResult;
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
    pub verify_access_token: Arc<dyn VerifyAccessTokenUseCase>,
}

#[derive(Clone)]
pub struct UsersState {
    pub get: Arc<dyn GetUserUseCase>,
    pub search: Arc<dyn SearchUseCase<UserSearchField, UserSearchResult>>,
}

#[derive(Clone)]
pub struct AppState {
    pub rate_limiter: Arc<dyn RateLimiterPort>,
    pub system: SystemState,
    pub auth: AuthState,
    pub users: UsersState,
}
