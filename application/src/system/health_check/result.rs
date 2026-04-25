#[derive(Clone, Debug)]
pub struct HealthCheckResult {
    pub postgresql_async: String,
    pub redis_async: String,
}
