use crate::auth::refresh::result::RefreshResult;
use crate::errors::ServiceError;
use async_trait::async_trait;

#[async_trait]
pub trait RefreshUseCase: Send + Sync {
    async fn refresh(&self, refresh_token: String) -> Result<RefreshResult, ServiceError>;
}
