use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
pub struct HealthCheckResponse {
    #[schema(example = "2026-03-28 19:26:52.972609+00")]
    pub postgresql_async: String,
    #[schema(example = "PONG")]
    pub redis_async: String,
}
