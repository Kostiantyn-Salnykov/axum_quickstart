use std::sync::Arc;

use crate::errors::ServiceError;
use crate::users::get::result::UserResult;
use crate::users::get::use_case::GetUserUseCase;
use crate::users::user_repository_port::UserRepositoryPort;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetUserService {
    users: Arc<dyn UserRepositoryPort>,
}

impl GetUserService {
    pub fn new(users: Arc<dyn UserRepositoryPort>) -> Self {
        Self { users }
    }
}

#[async_trait]
impl GetUserUseCase for GetUserService {
    async fn get_by_id(&self, id: Uuid) -> Result<UserResult, ServiceError> {
        let user = self
            .users
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound)?;
        Ok(UserResult::from(user))
    }

    async fn get_me(&self, user_id: Uuid) -> Result<UserResult, ServiceError> {
        let user = self
            .users
            .find_by_id(user_id)
            .await?
            .ok_or(ServiceError::InvalidCredentials)?;

        Ok(UserResult::from(user))
    }
}
