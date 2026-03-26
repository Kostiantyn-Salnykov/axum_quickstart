use crate::errors::ServiceError;
use async_trait::async_trait;
use domain::entities::user::user::User;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ServiceError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, ServiceError>;
    async fn create(&self, user: &User) -> Result<User, ServiceError>;
    async fn update(&self, user: &User) -> Result<User, ServiceError>;
}

#[async_trait]
pub trait PasswordHasher: Send + Sync {
    fn hash(&self, plaintext: &str) -> Result<String, ServiceError>;
    fn verify(&self, plaintext: &str, hash: &str) -> Result<bool, ServiceError>;
}
