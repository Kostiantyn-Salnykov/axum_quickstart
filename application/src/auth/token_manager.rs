use crate::errors::ServiceError;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenAudience {
    Access,
    Refresh,
    EmailConfirm,
    PasswordReset,
}

impl TokenAudience {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Access => "access",
            Self::Refresh => "refresh",
            Self::EmailConfirm => "email_confirm",
            Self::PasswordReset => "password_reset",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenPayload {
    pub user_id: Uuid,
    pub audience: TokenAudience,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub access_expires_at: DateTime<Utc>,
    pub refresh_expires_at: DateTime<Utc>,
}

pub trait TokenManager: Send + Sync {
    fn issue_access_token(&self, user_id: Uuid) -> Result<(String, DateTime<Utc>), ServiceError>;
    fn issue_refresh_token(&self, user_id: Uuid) -> Result<(String, DateTime<Utc>), ServiceError>;
    fn verify(&self, token: &str) -> Result<TokenPayload, ServiceError>;

    fn issue_token_pair(&self, user_id: Uuid) -> Result<TokenPair, ServiceError> {
        let (access_token, access_expires_at) = self.issue_access_token(user_id)?;
        let (refresh_token, refresh_expires_at) = self.issue_refresh_token(user_id)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            access_expires_at,
            refresh_expires_at,
        })
    }
}
