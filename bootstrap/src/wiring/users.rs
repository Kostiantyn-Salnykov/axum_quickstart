use std::sync::Arc;

use application::search::repository::SearchRepositoryPort;
use application::search::service::SearchService;
use application::search::use_case::SearchUseCase;
use application::users::get::service::GetUserService;
use application::users::get::use_case::GetUserUseCase;
use application::users::search::query::UserSearchField;
use application::users::search::result::UserSearchResult;
use application::users::user_repository_port::UserRepositoryPort;

pub fn build_get_user_service(users: Arc<dyn UserRepositoryPort>) -> Arc<dyn GetUserUseCase> {
    Arc::new(GetUserService::new(users))
}

pub fn build_search_user_service(
    users: Arc<dyn SearchRepositoryPort<UserSearchField, UserSearchResult>>,
) -> Arc<dyn SearchUseCase<UserSearchField, UserSearchResult>> {
    Arc::new(SearchService::new(users))
}
