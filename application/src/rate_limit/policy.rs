use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateLimitAlgorithm {
    FixedWindow,
    SlidingWindow,
    TokenBucket,
}

impl fmt::Display for RateLimitAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FixedWindow => write!(f, "fixed window"),
            Self::SlidingWindow => write!(f, "sliding window"),
            Self::TokenBucket => write!(f, "token bucket"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimitInfo {
    pub algorithm: RateLimitAlgorithm,
    pub limit: u64,
    pub remaining: u64,
    pub reset_after_seconds: u64,
}

impl RateLimitInfo {
    pub const fn new(
        algorithm: RateLimitAlgorithm,
        limit: u64,
        remaining: u64,
        reset_after_seconds: u64,
    ) -> Self {
        Self {
            algorithm,
            limit,
            remaining,
            reset_after_seconds,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateLimitWindow {
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
}

impl RateLimitWindow {
    pub const fn seconds(value: u64) -> Self {
        Self::Seconds(value)
    }

    pub const fn minutes(value: u64) -> Self {
        Self::Minutes(value)
    }

    pub const fn hours(value: u64) -> Self {
        Self::Hours(value)
    }

    pub const fn days(value: u64) -> Self {
        Self::Days(value)
    }

    pub const fn as_seconds(self) -> u64 {
        match self {
            Self::Seconds(value) => value,
            Self::Minutes(value) => value * 60,
            Self::Hours(value) => value * 60 * 60,
            Self::Days(value) => value * 60 * 60 * 24,
        }
    }

    pub const fn as_millis(self) -> u64 {
        self.as_seconds() * 1000
    }
}

impl fmt::Display for RateLimitWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (value, unit) = match self {
            Self::Seconds(value) => (*value, "second"),
            Self::Minutes(value) => (*value, "minute"),
            Self::Hours(value) => (*value, "hour"),
            Self::Days(value) => (*value, "day"),
        };
        let suffix = if value == 1 { "" } else { "s" };
        write!(f, "{value} {unit}{suffix}")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RateLimitPolicy {
    pub algorithm: RateLimitAlgorithm,
    pub max_attempts: u64,
    pub window: RateLimitWindow,
}

impl RateLimitPolicy {
    pub const fn fixed_window(max_attempts: u64, window: RateLimitWindow) -> Self {
        Self {
            algorithm: RateLimitAlgorithm::FixedWindow,
            max_attempts,
            window,
        }
    }

    pub const fn sliding_window(max_attempts: u64, window: RateLimitWindow) -> Self {
        Self {
            algorithm: RateLimitAlgorithm::SlidingWindow,
            max_attempts,
            window,
        }
    }

    pub const fn token_bucket(max_attempts: u64, window: RateLimitWindow) -> Self {
        Self {
            algorithm: RateLimitAlgorithm::TokenBucket,
            max_attempts,
            window,
        }
    }
}
