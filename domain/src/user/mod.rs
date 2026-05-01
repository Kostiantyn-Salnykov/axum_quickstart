use chrono::{DateTime, Utc};
use uuid::Uuid;

pub mod email;
pub mod password_hash;
pub mod phone;
pub mod provider;
pub mod raw_password;
pub mod status;

use self::email::Email;
use self::password_hash::PasswordHash;
use self::phone::Phone;
use self::provider::AuthProvider;
use self::status::UserStatus;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: Email,
    pub phone: Option<Phone>,
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
            phone: None,
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
            phone: None,
            password_hash: None,
            status: UserStatus::Confirmed,
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

    pub fn set_phone(&mut self, phone: Option<Phone>) {
        self.phone = phone;
        self.updated_at = Utc::now();
    }

    pub fn confirm(&mut self) {
        self.status = UserStatus::Confirmed;
        self.updated_at = Utc::now();
    }

    pub fn require_password_reset(&mut self) {
        self.status = UserStatus::ForceChangePassword;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_local_user_with_expected_defaults() {
        let email = Email::new("local@example.com").unwrap();
        let password_hash = PasswordHash::from("hashed-password".to_string());

        let user = User::new_local(email.clone(), password_hash);

        assert_eq!(user.email, email);
        assert_eq!(user.first_name, "");
        assert_eq!(user.last_name, "");
        assert!(user.phone.is_none());
        assert!(user.provider.is_none());
        assert!(user.password_hash.is_some());
        assert_eq!(user.status, UserStatus::Unconfirmed);
        assert!(user.updated_at >= user.created_at);
    }

    #[test]
    fn creates_external_user_with_expected_defaults() {
        let email = Email::new("external@example.com").unwrap();

        let user = User::new_external(email.clone(), AuthProvider::Google);

        assert_eq!(user.email, email);
        assert!(user.password_hash.is_none());
        assert_eq!(user.provider, Some(AuthProvider::Google));
        assert_eq!(user.status, UserStatus::Confirmed);
    }

    #[test]
    fn mutating_methods_update_fields_and_timestamp() {
        let email = Email::new("user@example.com").unwrap();
        let password_hash = PasswordHash::from("hashed-password".to_string());
        let mut user = User::new_local(email, password_hash);

        let initial_updated_at = user.updated_at;
        user.set_name("John".to_string(), "Doe".to_string());
        assert_eq!(user.first_name, "John");
        assert_eq!(user.last_name, "Doe");
        assert!(user.updated_at >= initial_updated_at);

        let after_name_update = user.updated_at;
        let phone = Phone::new("+12025550188").unwrap();
        user.set_phone(Some(phone.clone()));
        assert_eq!(user.phone, Some(phone));
        assert!(user.updated_at >= after_name_update);

        user.confirm();
        assert_eq!(user.status, UserStatus::Confirmed);

        user.require_password_reset();
        assert_eq!(user.status, UserStatus::ForceChangePassword);
    }
}
