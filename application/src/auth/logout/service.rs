use std::sync::Arc;

use crate::auth::logout::use_case::LogoutUseCase;
use crate::auth::token_blacklist_port::TokenBlacklistPort;
use crate::errors::ServiceError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct LogoutService {
    blacklist: Arc<dyn TokenBlacklistPort>,
}

impl LogoutService {
    pub fn new(blacklist: Arc<dyn TokenBlacklistPort>) -> Self {
        Self { blacklist }
    }
}

#[async_trait]
impl LogoutUseCase for LogoutService {
    async fn logout(&self, token: String, expires_at: DateTime<Utc>) -> Result<(), ServiceError> {
        let token = token.trim();
        tracing::debug!("Attempting to revoke current bearer token.");
        self.blacklist.revoke_until(token, expires_at).await
    }
}
