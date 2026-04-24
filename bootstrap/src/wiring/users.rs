use std::sync::Arc;

use application::auth::token_manager::TokenManager;
use application::users::get::inbound::GetUserUseCase;
use application::users::get::service::GetUserService;
use application::users::user_repository::UserRepository;

pub fn build_get_user_service(
    users: Arc<dyn UserRepository>,
    token_manager: Arc<dyn TokenManager>,
) -> Arc<dyn GetUserUseCase> {
    Arc::new(GetUserService::new(users, token_manager))
}
