use crate::errors::ServiceError;
use crate::search::query::SearchQuery;
use crate::search::result::SearchPageResult;
use crate::users::search::query::UserSearchField;
use crate::users::search::result::UserSearchResult;
use async_trait::async_trait;
use domain::user::User;
use uuid::Uuid;

#[async_trait]
pub trait UserRepositoryPort: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ServiceError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, ServiceError>;
    async fn search(
        &self,
        query: SearchQuery<UserSearchField>,
    ) -> Result<SearchPageResult<UserSearchResult>, ServiceError>;
    async fn create(&self, user: &User) -> Result<User, ServiceError>;
    async fn update(&self, user: &User) -> Result<User, ServiceError>;
}
