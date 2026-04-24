use std::sync::Arc;

use application::auth::login::inbound::LoginUseCase;
use application::auth::login::service::LoginService;
use application::auth::logout::inbound::LogoutUseCase;
use application::auth::logout::service::LogoutService;
use application::auth::password_hasher::PasswordHasher;
use application::auth::refresh::inbound::RefreshUseCase;
use application::auth::refresh::service::RefreshService;
use application::auth::register::inbound::RegisterUseCase;
use application::auth::register::service::RegisterService;
use application::auth::token_blacklist::TokenBlacklist;
use application::auth::token_manager::TokenManager;
use application::users::user_repository::UserRepository;

pub struct AuthServices {
    pub register: Arc<dyn RegisterUseCase>,
    pub login: Arc<dyn LoginUseCase>,
    pub logout: Arc<dyn LogoutUseCase>,
    pub refresh: Arc<dyn RefreshUseCase>,
}

pub fn build_auth_services(
    users: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    token_manager: Arc<dyn TokenManager>,
    blacklist: Arc<dyn TokenBlacklist>,
) -> AuthServices {
    AuthServices {
        register: Arc::new(RegisterService::new(users.clone(), password_hasher.clone())),
        login: Arc::new(LoginService::new(
            users.clone(),
            password_hasher,
            token_manager.clone(),
        )),
        logout: Arc::new(LogoutService::new(blacklist)),
        refresh: Arc::new(RefreshService::new(users, token_manager)),
    }
}
