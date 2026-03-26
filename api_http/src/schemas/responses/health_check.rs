#[derive(serde::Serialize)]
pub struct HealthCheckResponse {
    pub postgresql_async: String,
}
