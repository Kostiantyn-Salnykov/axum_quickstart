use crate::errors::AppError;
use crate::response::JsendResponse;
use crate::state::AppState;
use crate::users::register::request::RegisterUserRequest;
use crate::users::register::response::RegisterUserResponse;
use application::errors::ServiceError;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = state
        .register_user
        .register(
            payload.email,
            payload.password,
            payload.first_name,
            payload.last_name,
        )
        .await
        .map_err(map_service_error)?;

    Ok(JsendResponse::success(
        StatusCode::CREATED,
        RegisterUserResponse::from(user),
        "User registered successfully.",
    ))
}

fn map_service_error(error: ServiceError) -> AppError {
    match error {
        ServiceError::Validation(message) => AppError::Validation(message),
        ServiceError::Conflict(message) => AppError::Conflict(message),
        ServiceError::NotFound => AppError::NotFound("Resource not found.".to_string()),
        ServiceError::InvalidCredentials => AppError::Unauthorized,
        ServiceError::Internal => AppError::Internal(anyhow::anyhow!("Internal server error")),
    }
}
