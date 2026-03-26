use std::sync::Arc;

use crate::errors::ServiceError;
use crate::ports::user::{PasswordHasher, UserRepository};
use crate::use_cases::register_user::RegisterUserUseCase;
use async_trait::async_trait;
use domain::entities::user::email::Email;
use domain::entities::user::password_hash::PasswordHash;
use domain::entities::user::user::User;

#[derive(Clone)]
pub struct RegisterUserService {
    users: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl RegisterUserService {
    pub fn new(users: Arc<dyn UserRepository>, password_hasher: Arc<dyn PasswordHasher>) -> Self {
        Self {
            users,
            password_hasher,
        }
    }
}

#[async_trait]
impl RegisterUserUseCase for RegisterUserService {
    async fn register(
        &self,
        email: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<User, ServiceError> {
        let email = Email::new(&email).map_err(|e| ServiceError::Validation(e.to_string()))?;
        let password = password.trim().to_string();

        if password.len() < 8 {
            return Err(ServiceError::Validation(
                "Password must be at least 8 characters long.".to_string(),
            ));
        }

        if self.users.find_by_email(email.as_str()).await?.is_some() {
            return Err(ServiceError::Conflict(
                "User with this email already exists.".to_string(),
            ));
        }

        let hash = self.password_hasher.hash(&password)?;
        let mut user = User::new_local(email, PasswordHash::from_hash(hash));

        if let Some(first_name) = first_name.map(|v| v.trim().to_string()) {
            if !first_name.is_empty() {
                user.first_name = first_name;
            }
        }

        if let Some(last_name) = last_name.map(|v| v.trim().to_string()) {
            if !last_name.is_empty() {
                user.last_name = last_name;
            }
        }

        self.users.create(&user).await
    }
}
