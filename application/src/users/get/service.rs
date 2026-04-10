use std::sync::Arc;

use crate::auth::token_manager::{TokenAudience, TokenManager};
use crate::errors::ServiceError;
use crate::users::get::inbound::GetUser;
use crate::users::get::result::UserResult;
use crate::users::user_repository::UserRepository;
use async_trait::async_trait;
use domain::user::user::User;
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

    fn to_result(user: User) -> UserResult {
        UserResult {
            id: user.id,
            email: user.email.to_owned(),
            first_name: user.first_name,
            last_name: user.last_name,
            status: user.status.as_str().to_owned(),
            provider: user.provider.map(|provider| provider.as_str().to_owned()),
        }
    }
}

#[async_trait]
impl GetUser for GetUserService {
    async fn get_by_id(&self, id: Uuid) -> Result<UserResult, ServiceError> {
        let user = self
            .users
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound)?;
        Ok(Self::to_result(user))
    }

    async fn get_me(&self, access_token: String) -> Result<UserResult, ServiceError> {
        let payload = self.token_manager.verify(access_token.trim())?;
        if payload.audience != TokenAudience::Access {
            return Err(ServiceError::InvalidCredentials);
        }

        let user = self
            .users
            .find_by_id(payload.user_id)
            .await?
            .ok_or(ServiceError::InvalidCredentials)?;

        Ok(Self::to_result(user))
    }
}
