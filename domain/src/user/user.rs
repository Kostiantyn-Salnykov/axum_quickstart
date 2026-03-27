use super::email::Email;
use super::password_hash::PasswordHash;
use super::provider::AuthProvider;
use super::status::UserStatus;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: Email,
    pub password_hash: Option<PasswordHash>,
    pub status: UserStatus,
    pub provider: Option<AuthProvider>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new_local(email: Email, password_hash: PasswordHash) -> Self {
        Self {
            id: Uuid::now_v7(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            email,
            password_hash: Some(password_hash),
            status: UserStatus::Unconfirmed,
            provider: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn new_external(email: Email, provider: AuthProvider) -> Self {
        Self {
            id: Uuid::now_v7(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            email,
            password_hash: None,
            status: UserStatus::ExternalProvider,
            provider: Some(provider),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn set_name(&mut self, first_name: String, last_name: String) {
        self.first_name = first_name;
        self.last_name = last_name;
        self.updated_at = Utc::now();
    }

    pub fn confirm(&mut self) {
        self.status = UserStatus::Confirmed;
        self.updated_at = Utc::now();
    }

    pub fn require_password_reset(&mut self) {
        self.status = UserStatus::ResetRequired;
        self.updated_at = Utc::now();
    }
}
