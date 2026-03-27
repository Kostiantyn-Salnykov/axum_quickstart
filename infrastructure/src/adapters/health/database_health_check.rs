use application::errors::ServiceError;
use application::system::health_check::outbound::DatabaseHealthCheck;
use async_trait::async_trait;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};

pub struct SeaOrmDatabaseHealthCheck {
    db: DatabaseConnection,
}

impl SeaOrmDatabaseHealthCheck {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl DatabaseHealthCheck for SeaOrmDatabaseHealthCheck {
    async fn current_timestamp(&self) -> Result<String, ServiceError> {
        let stmt = Statement::from_string(
            DbBackend::Postgres,
            "SELECT CURRENT_TIMESTAMP::text AS current_timestamp".to_string(),
        );

        let row = self
            .db
            .query_one_raw(stmt)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to return CURRENT_TIMESTAMP.");
                ServiceError::Internal
            })?
            .ok_or_else(|| ServiceError::Internal)?;

        let ts: String = row.try_get("", "current_timestamp").map_err(|e| {
            tracing::error!(error = %e, "Cannot retrieve `current_timestamp` from query result.");
            ServiceError::Internal
        })?;

        Ok(ts)
    }
}
