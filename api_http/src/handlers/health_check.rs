use crate::errors::AppError;
use crate::schemas::responses::JsendResponse;
use crate::schemas::responses::health_check::HealthCheckResponse;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn health_check(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let result = state
        .health_check_service
        .check()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

    Ok(JsendResponse::success(
        StatusCode::OK,
        HealthCheckResponse {
            postgresql_async: result,
        },
        "Health check response from DB.",
    ))
}
