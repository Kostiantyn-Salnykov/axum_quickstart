use std::sync::Arc;

use crate::auth::refresh::inbound::Refresh;
use crate::auth::refresh::result::RefreshResult;
use crate::auth::token_manager::{TokenAudience, TokenManager};
use crate::auth::user_repository::UserRepository;
use crate::errors::ServiceError;
use async_trait::async_trait;

#[derive(Clone)]
pub struct RefreshService {
    users: Arc<dyn UserRepository>,
    token_manager: Arc<dyn TokenManager>,
}

impl RefreshService {
    pub fn new(users: Arc<dyn UserRepository>, token_manager: Arc<dyn TokenManager>) -> Self {
        Self {
            users,
            token_manager,
        }
    }
}

#[async_trait]
impl Refresh for RefreshService {
    async fn refresh(&self, refresh_token: String) -> Result<RefreshResult, ServiceError> {
        let payload = self.token_manager.verify(refresh_token.trim())?;
        if payload.audience != TokenAudience::Refresh {
            return Err(ServiceError::InvalidCredentials);
        }

        let user = self
            .users
            .find_by_id(payload.user_id)
            .await?
            .ok_or(ServiceError::InvalidCredentials)?;

        if !user.status.can_login() {
            return Err(ServiceError::InvalidCredentials);
        }

        let pair = self.token_manager.issue_token_pair(user.id)?;
        Ok(RefreshResult {
            user_id: user.id,
            access_token: pair.access_token,
            refresh_token: pair.refresh_token,
            access_expires_at: pair.access_expires_at,
            refresh_expires_at: pair.refresh_expires_at,
        })
    }
}
