use application::errors::ServiceError;
use application::system::health_check::port::HealthCheckPort;
use application::system::health_check::result::HealthCheckResult;
use async_trait::async_trait;

use crate::adapters::health::database_health_check::SeaOrmDatabaseHealthCheck;
use crate::adapters::health::redis_health_check::RedisHealthCheck;

pub struct CompositeHealthCheck {
    database: SeaOrmDatabaseHealthCheck,
    redis: RedisHealthCheck,
}

impl CompositeHealthCheck {
    pub fn new(database: SeaOrmDatabaseHealthCheck, redis: RedisHealthCheck) -> Self {
        Self { database, redis }
    }
}

#[async_trait]
impl HealthCheckPort for CompositeHealthCheck {
    async fn check(&self) -> Result<HealthCheckResult, ServiceError> {
        let postgresql_async = self.database.check().await?;
        let redis_async = self.redis.check().await?;

        Ok(HealthCheckResult {
            postgresql_async,
            redis_async,
        })
    }
}
