use std::sync::Arc;

use crate::auth::logout::inbound::Logout;
use crate::auth::token_blacklist::TokenBlacklist;
use crate::errors::ServiceError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct LogoutService {
    blacklist: Arc<dyn TokenBlacklist>,
}

impl LogoutService {
    pub fn new(blacklist: Arc<dyn TokenBlacklist>) -> Self {
        Self { blacklist }
    }
}

#[async_trait]
impl Logout for LogoutService {
    async fn logout(&self, token: String, expires_at: DateTime<Utc>) -> Result<(), ServiceError> {
        let token = token.trim();
        tracing::debug!("Attempting to revoke current bearer token.");
        self.blacklist.revoke_until(token, expires_at).await
    }
}
