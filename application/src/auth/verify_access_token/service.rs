use std::sync::Arc;

use crate::auth::token_manager_port::{TokenAudience, TokenManagerPort, TokenPayload};
use crate::auth::verify_access_token::use_case::VerifyAccessTokenUseCase;
use crate::errors::ServiceError;
use async_trait::async_trait;

#[derive(Clone)]
pub struct VerifyAccessTokenService {
    token_manager: Arc<dyn TokenManagerPort>,
}

impl VerifyAccessTokenService {
    pub fn new(token_manager: Arc<dyn TokenManagerPort>) -> Self {
        Self { token_manager }
    }
}

#[async_trait]
impl VerifyAccessTokenUseCase for VerifyAccessTokenService {
    async fn verify_access_token(&self, token: String) -> Result<TokenPayload, ServiceError> {
        let payload = self.token_manager.verify(token.trim()).await?;
        if payload.audience != TokenAudience::Access {
            return Err(ServiceError::InvalidCredentials);
        }

        Ok(payload)
    }
}
