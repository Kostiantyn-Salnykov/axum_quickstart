use std::collections::HashMap;

use application::errors::ServiceError;
use application::rate_limit::policy::{RateLimitAlgorithm, RateLimitInfo, RateLimitPolicy};
use application::rate_limit::rate_limiter_port::RateLimiterPort;
use async_trait::async_trait;
use chrono::Utc;
use redis::{
    AsyncCommands, ErrorKind, RedisError, ServerErrorKind, aio::ConnectionManager, cmd, pipe,
};

use crate::adapters::redis::client::RedisClient;

#[derive(Clone)]
pub struct RedisRateLimiterAdapter {
    client: RedisClient,
}

impl RedisRateLimiterAdapter {
    pub fn new(client: RedisClient) -> Self {
        Self { client }
    }

    fn algorithm_slug(algorithm: RateLimitAlgorithm) -> &'static str {
        match algorithm {
            RateLimitAlgorithm::FixedWindow => "fixed",
            RateLimitAlgorithm::SlidingWindow => "sliding",
            RateLimitAlgorithm::TokenBucket => "bucket",
        }
    }

    fn limiter_key(scope: &str, key: &str, policy: RateLimitPolicy) -> String {
        format!(
            "ratelimit:{scope}:{}:{}:{}:{key}",
            Self::algorithm_slug(policy.algorithm),
            policy.max_attempts,
            policy.window.as_seconds()
        )
    }

    fn allowed(policy: RateLimitPolicy, remaining: u64, reset_after_seconds: u64) -> RateLimitInfo {
        RateLimitInfo::new(
            policy.algorithm,
            policy.max_attempts,
            remaining,
            reset_after_seconds,
        )
    }

    fn rate_limited_error(policy: RateLimitPolicy, info: RateLimitInfo) -> ServiceError {
        ServiceError::RateLimited {
            info,
            message: format!(
                "Too many requests. Please try again in {} using {}.",
                policy.window, policy.algorithm
            ),
        }
    }

    fn transaction_aborted(error: &RedisError) -> bool {
        matches!(error.kind(), ErrorKind::Server(ServerErrorKind::ExecAbort))
    }

    async fn watch_keys(
        connection: &mut ConnectionManager,
        keys: &[&str],
    ) -> Result<(), RedisError> {
        let mut command = cmd("WATCH");
        command.arg(keys);
        command.query_async(connection).await
    }

    async fn unwatch(connection: &mut ConnectionManager) {
        let _: redis::RedisResult<()> = cmd("UNWATCH").query_async(connection).await;
    }

    async fn fixed_window(
        &self,
        connection: &mut ConnectionManager,
        scope: &str,
        key: &str,
        policy: RateLimitPolicy,
    ) -> Result<RateLimitInfo, ServiceError> {
        let limiter_key = Self::limiter_key(scope, key, policy);
        let window_seconds = policy.window.as_seconds();

        for _ in 0..16 {
            let mut watched = connection.clone();
            Self::watch_keys(&mut watched, &[&limiter_key])
                .await
                .map_err(|error| {
                    Self::map_redis_error(
                        error,
                        &watched,
                        scope,
                        &limiter_key,
                        policy,
                        "watch fixed-window",
                    )
                })?;

            let current: Option<u64> = watched.get(&limiter_key).await.map_err(|error| {
                Self::map_redis_error(
                    error,
                    &watched,
                    scope,
                    &limiter_key,
                    policy,
                    "read fixed-window",
                )
            })?;
            let current = current.unwrap_or(0);
            let reset_after_seconds = if current == 0 {
                window_seconds
            } else {
                let ttl: i64 = watched.ttl(&limiter_key).await.map_err(|error| {
                    Self::map_redis_error(
                        error,
                        &watched,
                        scope,
                        &limiter_key,
                        policy,
                        "ttl fixed-window",
                    )
                })?;
                if ttl > 0 { ttl as u64 } else { window_seconds }
            };

            if current >= policy.max_attempts {
                Self::unwatch(&mut watched).await;
                return Err(Self::rate_limited_error(
                    policy,
                    Self::allowed(policy, 0, reset_after_seconds),
                ));
            }

            let mut pipeline = pipe();
            pipeline.atomic();
            if current == 0 {
                pipeline
                    .cmd("SET")
                    .arg(&limiter_key)
                    .arg(1_u64)
                    .arg("EX")
                    .arg(window_seconds)
                    .ignore();
            } else {
                pipeline.cmd("INCR").arg(&limiter_key).ignore();
            }

            let exec_result: redis::RedisResult<()> = pipeline.query_async(&mut watched).await;
            Self::unwatch(&mut watched).await;

            match exec_result {
                Ok(()) => {
                    let remaining = policy.max_attempts.saturating_sub(current + 1);
                    return Ok(Self::allowed(policy, remaining, reset_after_seconds));
                }
                Err(error) if Self::transaction_aborted(&error) => continue,
                Err(error) => {
                    return Err(Self::map_redis_error(
                        error,
                        &watched,
                        scope,
                        &limiter_key,
                        policy,
                        "fixed-window transaction",
                    ));
                }
            }
        }

        Err(ServiceError::internal(anyhow::anyhow!(
            "fixed-window rate limit transaction retried too many times"
        )))
    }

    async fn sliding_window(
        &self,
        connection: &mut ConnectionManager,
        scope: &str,
        key: &str,
        policy: RateLimitPolicy,
    ) -> Result<RateLimitInfo, ServiceError> {
        let limiter_key = Self::limiter_key(scope, key, policy);
        let sequence_key = format!("{limiter_key}:seq");
        let window_ms = policy.window.as_millis();
        let window_seconds = policy.window.as_seconds();

        for _ in 0..16 {
            let mut watched = connection.clone();
            Self::watch_keys(&mut watched, &[&limiter_key, &sequence_key])
                .await
                .map_err(|error| {
                    Self::map_redis_error(
                        error,
                        &watched,
                        scope,
                        &limiter_key,
                        policy,
                        "watch sliding-window",
                    )
                })?;

            let now_ms = Utc::now().timestamp_millis();
            let cutoff = now_ms.saturating_sub(window_ms as i64);

            let _: i64 = cmd("ZREMRANGEBYSCORE")
                .arg(&limiter_key)
                .arg(0)
                .arg(cutoff)
                .query_async(&mut watched)
                .await
                .map_err(|error| {
                    Self::map_redis_error(
                        error,
                        &watched,
                        scope,
                        &limiter_key,
                        policy,
                        "cleanup sliding-window",
                    )
                })?;
            let current: i64 = cmd("ZCARD")
                .arg(&limiter_key)
                .query_async(&mut watched)
                .await
                .map_err(|error| {
                    Self::map_redis_error(
                        error,
                        &watched,
                        scope,
                        &limiter_key,
                        policy,
                        "count sliding-window",
                    )
                })?;

            let reset_after_seconds = if current == 0 {
                window_seconds
            } else {
                let oldest: Vec<(String, f64)> = cmd("ZRANGE")
                    .arg(&limiter_key)
                    .arg(0)
                    .arg(0)
                    .arg("WITHSCORES")
                    .query_async(&mut watched)
                    .await
                    .map_err(|error| {
                        Self::map_redis_error(
                            error,
                            &watched,
                            scope,
                            &limiter_key,
                            policy,
                            "read sliding-window oldest",
                        )
                    })?;
                oldest
                    .first()
                    .map(|(_, score)| {
                        let expires_in_ms =
                            ((*score as i64) + window_ms as i64 - now_ms).max(0) as u64;
                        expires_in_ms.div_ceil(1000).max(1)
                    })
                    .unwrap_or(window_seconds)
            };

            if (current as u64) >= policy.max_attempts {
                Self::unwatch(&mut watched).await;
                return Err(Self::rate_limited_error(
                    policy,
                    Self::allowed(policy, 0, reset_after_seconds),
                ));
            }

            let sequence: i64 = watched.incr(&sequence_key, 1).await.map_err(|error| {
                Self::map_redis_error(
                    error,
                    &watched,
                    scope,
                    &limiter_key,
                    policy,
                    "sequence sliding-window",
                )
            })?;
            let member = format!("{now_ms}-{sequence}");

            let mut pipeline = pipe();
            pipeline.atomic();
            pipeline
                .cmd("ZADD")
                .arg(&limiter_key)
                .arg(now_ms)
                .arg(member)
                .ignore();
            pipeline
                .cmd("PEXPIRE")
                .arg(&limiter_key)
                .arg(window_ms as i64)
                .ignore();
            pipeline
                .cmd("PEXPIRE")
                .arg(&sequence_key)
                .arg((window_ms.saturating_mul(2)) as i64)
                .ignore();

            let exec_result: redis::RedisResult<()> = pipeline.query_async(&mut watched).await;
            Self::unwatch(&mut watched).await;

            match exec_result {
                Ok(()) => {
                    let remaining = policy.max_attempts.saturating_sub(current as u64 + 1);
                    return Ok(Self::allowed(policy, remaining, reset_after_seconds));
                }
                Err(error) if Self::transaction_aborted(&error) => continue,
                Err(error) => {
                    return Err(Self::map_redis_error(
                        error,
                        &watched,
                        scope,
                        &limiter_key,
                        policy,
                        "sliding-window transaction",
                    ));
                }
            }
        }

        Err(ServiceError::internal(anyhow::anyhow!(
            "sliding-window rate limit transaction retried too many times"
        )))
    }

    async fn token_bucket(
        &self,
        connection: &mut ConnectionManager,
        scope: &str,
        key: &str,
        policy: RateLimitPolicy,
    ) -> Result<RateLimitInfo, ServiceError> {
        let limiter_key = Self::limiter_key(scope, key, policy);
        let capacity = policy.max_attempts as f64;
        let window_ms = policy.window.as_millis() as f64;

        for _ in 0..16 {
            let mut watched = connection.clone();
            Self::watch_keys(&mut watched, &[&limiter_key])
                .await
                .map_err(|error| {
                    Self::map_redis_error(
                        error,
                        &watched,
                        scope,
                        &limiter_key,
                        policy,
                        "watch token-bucket",
                    )
                })?;

            let now_ms = Utc::now().timestamp_millis();
            let state: HashMap<String, String> =
                watched.hgetall(&limiter_key).await.map_err(|error| {
                    Self::map_redis_error(
                        error,
                        &watched,
                        scope,
                        &limiter_key,
                        policy,
                        "read token-bucket",
                    )
                })?;

            let tokens = state
                .get("tokens")
                .and_then(|value| value.parse::<f64>().ok())
                .unwrap_or(capacity);
            let ts = state
                .get("ts")
                .and_then(|value| value.parse::<i64>().ok())
                .unwrap_or(now_ms);
            let elapsed_ms = (now_ms - ts).max(0) as f64;
            let refill = (elapsed_ms * capacity) / window_ms;
            let refilled = (tokens + refill).min(capacity);
            let reset_after_seconds = if refilled >= capacity {
                0
            } else {
                (((capacity - refilled) * window_ms) / capacity / 1000.0)
                    .ceil()
                    .max(1.0) as u64
            };

            if refilled < 1.0 {
                Self::unwatch(&mut watched).await;
                return Err(Self::rate_limited_error(
                    policy,
                    Self::allowed(policy, 0, reset_after_seconds),
                ));
            }

            let tokens_after = refilled - 1.0;
            let mut pipeline = pipe();
            pipeline.atomic();
            pipeline
                .cmd("HSET")
                .arg(&limiter_key)
                .arg("tokens")
                .arg(tokens_after)
                .arg("ts")
                .arg(now_ms)
                .ignore();
            pipeline
                .cmd("PEXPIRE")
                .arg(&limiter_key)
                .arg((window_ms as u64).saturating_mul(2) as i64)
                .ignore();

            let exec_result: redis::RedisResult<()> = pipeline.query_async(&mut watched).await;
            Self::unwatch(&mut watched).await;

            match exec_result {
                Ok(()) => {
                    let remaining = tokens_after.floor().max(0.0) as u64;
                    return Ok(Self::allowed(policy, remaining, reset_after_seconds));
                }
                Err(error) if Self::transaction_aborted(&error) => continue,
                Err(error) => {
                    return Err(Self::map_redis_error(
                        error,
                        &watched,
                        scope,
                        &limiter_key,
                        policy,
                        "token-bucket transaction",
                    ));
                }
            }
        }

        Err(ServiceError::internal(anyhow::anyhow!(
            "token-bucket rate limit transaction retried too many times"
        )))
    }

    fn map_redis_error(
        error: RedisError,
        _connection: &ConnectionManager,
        scope: &str,
        limiter_key: &str,
        policy: RateLimitPolicy,
        context: &'static str,
    ) -> ServiceError {
        tracing::error!(
            error = ?error,
            scope = scope,
            key = %limiter_key,
            algorithm = %policy.algorithm,
            window = %policy.window,
            "Failed to execute {context}."
        );
        ServiceError::internal(error)
    }
}

#[async_trait]
impl RateLimiterPort for RedisRateLimiterAdapter {
    async fn check(
        &self,
        scope: &str,
        key: &str,
        policy: RateLimitPolicy,
    ) -> Result<RateLimitInfo, ServiceError> {
        let mut connection = self.client.connection().map_err(|error| {
            tracing::error!(
                error = ?error,
                redis_url = %self.client.url(),
                scope = scope,
                key = key,
                algorithm = %policy.algorithm,
                "Failed to acquire Redis connection for rate limit."
            );
            ServiceError::internal(error)
        })?;

        match policy.algorithm {
            RateLimitAlgorithm::FixedWindow => {
                self.fixed_window(&mut connection, scope, key, policy).await
            }
            RateLimitAlgorithm::SlidingWindow => {
                self.sliding_window(&mut connection, scope, key, policy)
                    .await
            }
            RateLimitAlgorithm::TokenBucket => {
                self.token_bucket(&mut connection, scope, key, policy).await
            }
        }
    }
}
