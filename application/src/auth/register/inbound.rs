use crate::auth::register::result::RegisterResult;
use crate::errors::ServiceError;
use async_trait::async_trait;

#[async_trait]
pub trait Register: Send + Sync {
    async fn register(
        &self,
        email: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<RegisterResult, ServiceError>;
}
