use crate::errors::ServiceError;
use crate::users::register::result::RegisterUserResult;
use async_trait::async_trait;

#[async_trait]
pub trait RegisterUser: Send + Sync {
    async fn register(
        &self,
        email: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<RegisterUserResult, ServiceError>;
}
