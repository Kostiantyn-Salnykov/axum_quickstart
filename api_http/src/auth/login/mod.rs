pub mod handler;
pub mod request;
pub mod response;
pub mod router;

use application::rate_limit::policy::RateLimitPolicy;
use application::rate_limit::policy::RateLimitWindow;

pub const LOGIN_RATE_LIMIT_POLICIES: [RateLimitPolicy; 1] = [RateLimitPolicy::token_bucket(
    5,
    RateLimitWindow::minutes(1),
)];

pub use router::router;
