use crate::errors::ServiceError;
use async_trait::async_trait;
use domain::entities::user::user::User;

#[async_trait]
pub trait RegisterUserUseCase: Send + Sync {
    async fn register(
        &self,
        email: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<User, ServiceError>;
}
