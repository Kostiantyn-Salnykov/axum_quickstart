use async_trait::async_trait;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use service::errors::ServiceError;
use service::ports::health_check::HealthCheckProvider;

pub struct DbHealthCheckProvider {
    db: DatabaseConnection,
}

impl DbHealthCheckProvider {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl HealthCheckProvider for DbHealthCheckProvider {
    async fn current_timestamp(&self) -> Result<String, ServiceError> {
        let stmt = Statement::from_string(
            DbBackend::Postgres,
            "SELECT CURRENT_TIMESTAMP::text AS current_timestamp".to_string(),
        );

        let row = self
            .db
            .query_one_raw(stmt)
            .await
            .map_err(|e| ServiceError::Infrastructure(e.to_string()))?
            .ok_or_else(|| ServiceError::Infrastructure("no row returned!".to_string()))?;

        let ts: String = row
            .try_get("", "current_timestamp")
            .map_err(|e| ServiceError::Infrastructure(e.to_string()))?;

        Ok(ts)
    }
}
