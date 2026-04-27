use std::sync::Arc;

use application::auth::login::use_case::LoginUseCase;
use application::auth::logout::use_case::LogoutUseCase;
use application::auth::refresh::use_case::RefreshUseCase;
use application::auth::register::use_case::RegisterUseCase;
use application::auth::token_manager::TokenManager;
use application::system::health_check::use_case::HealthCheckUseCase;
use application::users::get::use_case::GetUserUseCase;

pub struct SystemServices {
    pub health_check: Arc<dyn HealthCheckUseCase>,
}

pub struct AuthServices {
    pub register: Arc<dyn RegisterUseCase>,
    pub login: Arc<dyn LoginUseCase>,
    pub logout: Arc<dyn LogoutUseCase>,
    pub refresh: Arc<dyn RefreshUseCase>,
    pub token_manager: Arc<dyn TokenManager>,
}

pub struct UsersServices {
    pub get: Arc<dyn GetUserUseCase>,
}

pub struct ApplicationContainer {
    pub system: SystemServices,
    pub auth: AuthServices,
    pub users: UsersServices,
}

impl ApplicationContainer {
    pub fn new(
        health_check: Arc<dyn HealthCheckUseCase>,
        auth_register: Arc<dyn RegisterUseCase>,
        auth_login: Arc<dyn LoginUseCase>,
        auth_logout: Arc<dyn LogoutUseCase>,
        auth_refresh: Arc<dyn RefreshUseCase>,
        auth_token_manager: Arc<dyn TokenManager>,
        get_user: Arc<dyn GetUserUseCase>,
    ) -> Self {
        Self {
            system: SystemServices { health_check },
            auth: AuthServices {
                register: auth_register,
                login: auth_login,
                logout: auth_logout,
                refresh: auth_refresh,
                token_manager: auth_token_manager,
            },
            users: UsersServices { get: get_user },
        }
    }
}
