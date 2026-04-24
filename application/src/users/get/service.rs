use std::sync::Arc;

use crate::auth::token_manager::{TokenAudience, TokenManager};
use crate::errors::ServiceError;
use crate::users::get::inbound::GetUserUseCase;
use crate::users::get::result::UserResult;
use crate::users::user_repository::UserRepository;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetUserService {
    users: Arc<dyn UserRepository>,
    token_manager: Arc<dyn TokenManager>,
}

impl GetUserService {
    pub fn new(users: Arc<dyn UserRepository>, token_manager: Arc<dyn TokenManager>) -> Self {
        Self {
            users,
            token_manager,
        }
    }
}

#[async_trait]
impl GetUserUseCase for GetUserService {
    async fn get_by_id(&self, id: Uuid) -> Result<UserResult, ServiceError> {
        let user = self
            .users
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound)?;
        Ok(UserResult::from(user))
    }

    async fn get_me(&self, access_token: String) -> Result<UserResult, ServiceError> {
        let payload = self.token_manager.verify(access_token.trim()).await?;
        if payload.audience != TokenAudience::Access {
            return Err(ServiceError::InvalidCredentials);
        }

        let user = self
            .users
            .find_by_id(payload.user_id)
            .await?
            .ok_or(ServiceError::InvalidCredentials)?;

        Ok(UserResult::from(user))
    }
}
