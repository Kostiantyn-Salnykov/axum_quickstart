use std::sync::Arc;

use api_http::state::AuthState;
use application::auth::login::service::LoginService;
use application::auth::logout::service::LogoutService;
use application::auth::password_hasher_port::PasswordHasherPort;
use application::auth::refresh::service::RefreshService;
use application::auth::register::service::RegisterService;
use application::auth::token_blacklist_port::TokenBlacklistPort;
use application::auth::token_manager_port::TokenManagerPort;
use application::users::user_repository_port::UserRepositoryPort;

pub fn build_auth_services(
    users: Arc<dyn UserRepositoryPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    token_manager: Arc<dyn TokenManagerPort>,
    blacklist: Arc<dyn TokenBlacklistPort>,
) -> AuthState {
    AuthState {
        register: Arc::new(RegisterService::new(users.clone(), password_hasher.clone())),
        login: Arc::new(LoginService::new(
            users.clone(),
            password_hasher,
            token_manager.clone(),
        )),
        logout: Arc::new(LogoutService::new(blacklist)),
        refresh: Arc::new(RefreshService::new(users, token_manager.clone())),
        token_manager,
    }
}
