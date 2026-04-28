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

        if self.users.find_by_email(email.as_str()).await?.is_some() {
            return Err(ServiceError::Conflict(
                "User with this email already exists.".to_string(),
            ));
        }

        let hash = self.password_hasher.hash(password.as_str())?;
        let mut user = User::new_local(email, hash.into());
        user.set_phone(phone);

        if let Some(first_name) = first_name.map(|v| v.trim().to_string())
            && !first_name.is_empty()
        {
            user.first_name = first_name;
        }

        if let Some(last_name) = last_name.map(|v| v.trim().to_string())
            && !last_name.is_empty()
        {
            user.last_name = last_name;
        }

        user = self.users.create(&user).await?;
        Ok(user.into())
    }
}
