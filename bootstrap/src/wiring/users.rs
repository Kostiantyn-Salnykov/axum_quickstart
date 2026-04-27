use std::sync::Arc;

use application::auth::token_manager_port::TokenManagerPort;
use application::users::get::service::GetUserService;
use application::users::get::use_case::GetUserUseCase;
use application::users::user_repository_port::UserRepositoryPort;

pub fn build_get_user_service(
    users: Arc<dyn UserRepositoryPort>,
    token_manager: Arc<dyn TokenManagerPort>,
) -> Arc<dyn GetUserUseCase> {
    Arc::new(GetUserService::new(users, token_manager))
}
