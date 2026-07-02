use crate::errors::ServiceError;
use crate::rate_limit::policy::RateLimitPolicy;
use crate::rate_limit::rate_limiter_port::RateLimiterPort;

pub mod policy;
pub mod rate_limiter_port;

pub use policy::{RateLimitAlgorithm, RateLimitInfo, RateLimitWindow};

fn more_restrictive(current: RateLimitInfo, next: RateLimitInfo) -> RateLimitInfo {
    if next.remaining < current.remaining {
        return next;
    }

    if next.remaining > current.remaining {
        return current;
    }

    if next.reset_after_seconds > current.reset_after_seconds {
        return next;
    }

    current
}

pub async fn check_all(
    limiter: &dyn RateLimiterPort,
    scope: &str,
    key: &str,
    policies: impl IntoIterator<Item = RateLimitPolicy>,
) -> Result<RateLimitInfo, ServiceError> {
    let mut effective: Option<RateLimitInfo> = None;

    for policy in policies {
        let info = limiter.check(scope, key, policy).await?;
        effective = Some(match effective {
            Some(current) => more_restrictive(current, info),
            None => info,
        });
    }

    effective.ok_or_else(|| {
        ServiceError::internal(anyhow::anyhow!(
            "rate limiter check_all requires at least one policy"
        ))
    })
}
