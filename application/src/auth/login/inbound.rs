use crate::auth::login::result::LoginResult;
use crate::errors::ServiceError;
use async_trait::async_trait;

#[async_trait]
pub trait LoginUseCase: Send + Sync {
    async fn login(&self, email: String, password: String) -> Result<LoginResult, ServiceError>;
}
