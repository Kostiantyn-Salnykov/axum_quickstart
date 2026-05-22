use std::sync::Arc;

use crate::auth::password_hasher_port::PasswordHasherPort;
use crate::auth::register::result::RegisterResult;
use crate::auth::register::use_case::RegisterUseCase;
use crate::errors::ServiceError;
use crate::users::user_repository_port::UserRepositoryPort;
use async_trait::async_trait;
use domain::user::User;
use domain::user::email::Email;
use domain::user::phone::Phone;
use domain::user::raw_password::RawPassword;

#[derive(Clone)]
pub struct RegisterService {
    users: Arc<dyn UserRepositoryPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
}

impl RegisterService {
    pub fn new(
        users: Arc<dyn UserRepositoryPort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
    ) -> Self {
        Self {
            users,
            password_hasher,
        }
    }
}

#[async_trait]
impl RegisterUseCase for RegisterService {
    async fn register(
        &self,
        email: String,
        phone: Option<String>,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<RegisterResult, ServiceError> {
        let email = Email::new(&email).map_err(|e| ServiceError::Validation(e.to_string()))?;
        let phone = phone
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .map(|value| Phone::new(&value).map_err(|e| ServiceError::Validation(e.to_string())))
            .transpose()?;
        let password =
            RawPassword::new(&password).map_err(|e| ServiceError::Validation(e.to_string()))?;

        tracing::debug!(
            component = "RegisterService",
            method = "register",
            email = %email.as_str(),
            "Checking if user already exists."
        );
        if self.users.find_by_email(email.as_str()).await?.is_some() {
            tracing::warn!(
                component = "RegisterService",
                method = "register",
                email = %email.as_str(),
                "Registration rejected: user already exists."
            );

            return Err(ServiceError::Conflict(
                "User with this email already exists.".to_string(),
            ));
        }

        let hash = self.password_hasher.hash(password.as_str())?;
        let mut user = User::new_local(email, hash.into());
        user.set_phone(phone);

        let first_name = first_name
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());
        let last_name = last_name
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());
        if first_name.is_some() || last_name.is_some() {
            user.set_name(
                first_name.unwrap_or_default(),
                last_name.unwrap_or_default(),
            );
        }

        user = self.users.create(&user).await?;
        Ok(user.into())
    }
}
