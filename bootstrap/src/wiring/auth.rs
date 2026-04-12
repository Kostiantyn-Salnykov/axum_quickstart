use std::sync::Arc;

use application::auth::login::inbound::Login;
use application::auth::login::service::LoginService;
use application::auth::password_hasher::PasswordHasher;
use application::auth::refresh::inbound::Refresh;
use application::auth::refresh::service::RefreshService;
use application::auth::register::inbound::Register;
use application::auth::register::service::RegisterService;
use application::auth::token_manager::TokenManager;
use application::users::user_repository::UserRepository;

pub struct AuthServices {
    pub register: Arc<dyn Register>,
    pub login: Arc<dyn Login>,
    pub refresh: Arc<dyn Refresh>,
}

pub fn build_auth_services(
    users: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    token_manager: Arc<dyn TokenManager>,
) -> AuthServices {
    AuthServices {
        register: Arc::new(RegisterService::new(users.clone(), password_hasher.clone())),
        login: Arc::new(LoginService::new(
            users.clone(),
            password_hasher,
            token_manager.clone(),
        )),
        refresh: Arc::new(RefreshService::new(users, token_manager)),
    }
}
