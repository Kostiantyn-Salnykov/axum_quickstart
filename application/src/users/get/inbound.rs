use crate::errors::ServiceError;
use crate::users::get::result::UserResult;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait GetUserUseCase: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<UserResult, ServiceError>;
    async fn get_me(&self, access_token: String) -> Result<UserResult, ServiceError>;
}
