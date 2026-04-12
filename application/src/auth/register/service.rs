use std::sync::Arc;

use crate::auth::password_hasher::PasswordHasher;
use crate::auth::register::inbound::Register;
use crate::auth::register::result::RegisterResult;
use crate::errors::ServiceError;
use crate::users::user_repository::UserRepository;
use async_trait::async_trait;
use domain::user::email::Email;
use domain::user::user::User;

#[derive(Clone)]
pub struct RegisterService {
    users: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl RegisterService {
    pub fn new(users: Arc<dyn UserRepository>, password_hasher: Arc<dyn PasswordHasher>) -> Self {
        Self {
            users,
            password_hasher,
        }
    }
}

#[async_trait]
impl Register for RegisterService {
    async fn register(
        &self,
        email: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<RegisterResult, ServiceError> {
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
        let mut user = User::new_local(email, hash.into());

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
        Ok(user.into())
    }
}
