use crate::errors::ServiceError;
use async_trait::async_trait;
use domain::user::user::User;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ServiceError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, ServiceError>;
    async fn create(&self, user: &User) -> Result<User, ServiceError>;
    async fn update(&self, user: &User) -> Result<User, ServiceError>;
}
