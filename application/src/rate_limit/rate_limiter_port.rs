use crate::errors::ServiceError;
use crate::rate_limit::policy::{RateLimitInfo, RateLimitPolicy};
use async_trait::async_trait;

#[async_trait]
pub trait RateLimiterPort: Send + Sync {
    async fn check(
        &self,
        scope: &str,
        key: &str,
        policy: RateLimitPolicy,
    ) -> Result<RateLimitInfo, ServiceError>;
}
