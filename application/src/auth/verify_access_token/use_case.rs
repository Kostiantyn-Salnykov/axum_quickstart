use crate::auth::token_manager_port::TokenPayload;
use crate::errors::ServiceError;
use async_trait::async_trait;

#[async_trait]
pub trait VerifyAccessTokenUseCase: Send + Sync {
    async fn verify_access_token(&self, token: String) -> Result<TokenPayload, ServiceError>;
}
