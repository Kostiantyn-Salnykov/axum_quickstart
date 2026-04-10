use std::sync::Arc;

use crate::auth::login::inbound::Login;
use crate::auth::login::result::LoginResult;
use crate::auth::password_hasher::PasswordHasher;
use crate::auth::token_manager::TokenManager;
use crate::auth::user_repository::UserRepository;
use crate::errors::ServiceError;
use async_trait::async_trait;
use domain::user::email::Email;

#[derive(Clone)]
pub struct LoginService {
    users: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    token_manager: Arc<dyn TokenManager>,
}

impl LoginService {
    pub fn new(
        users: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
        token_manager: Arc<dyn TokenManager>,
    ) -> Self {
        Self {
            users,
            password_hasher,
            token_manager,
        }
    }
}

#[async_trait]
impl Login for LoginService {
    async fn login(&self, email: String, password: String) -> Result<LoginResult, ServiceError> {
        let email = Email::new(&email).map_err(|_| ServiceError::InvalidCredentials)?;
        let password = password.trim();

        let user = self
            .users
            .find_by_email(email.as_str())
            .await?
            .ok_or(ServiceError::InvalidCredentials)?;

        let hash = user
            .password_hash
            .as_ref()
            .ok_or(ServiceError::InvalidCredentials)?;

        let is_valid = self.password_hasher.verify(password, hash.as_str())?;
        if !is_valid || !user.status.can_login() {
            return Err(ServiceError::InvalidCredentials);
        }

        let pair = self.token_manager.issue_token_pair(user.id)?;
        Ok(LoginResult {
            user_id: user.id,
            access_token: pair.access_token,
            refresh_token: pair.refresh_token,
            access_expires_at: pair.access_expires_at,
            refresh_expires_at: pair.refresh_expires_at,
        })
    }
}
