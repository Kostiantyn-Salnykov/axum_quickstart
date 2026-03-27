use std::sync::Arc;

use crate::errors::ServiceError;
use crate::users::password_hasher::PasswordHasher;
use crate::users::register::inbound::RegisterUser;
use crate::users::register::result::RegisterUserResult;
use crate::users::user_repository::UserRepository;
use async_trait::async_trait;
use domain::user::email::Email;
use domain::user::password_hash::PasswordHash;
use domain::user::user::User;

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
impl RegisterUser for RegisterUserService {
    async fn register(
        &self,
        email: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<RegisterUserResult, ServiceError> {
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

        user = self.users.create(&user).await?;
        Ok(RegisterUserResult {
            id: user.id,
            email: user.email.to_owned(),
            first_name: user.first_name,
            last_name: user.last_name,
            status: user.status.as_str().to_owned(),
        })
    }
}
