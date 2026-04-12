use crate::auth::register::request::RegisterRequest;
use crate::auth::register::response::RegisterResponse;
use crate::errors::AppError;
use crate::responses::JsendResponse;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn register(
    State(state): State<AppState>,
    payload: Result<Json<RegisterRequest>, JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Json(payload) = payload.map_err(AppError::from_json_rejection)?;
    let result = state
        .auth
        .register
        .register(
            payload.email,
            payload.password,
            payload.first_name,
            payload.last_name,
        )
        .await?;

    Ok(JsendResponse::success(
        StatusCode::CREATED,
        RegisterResponse::from(result),
        "User registered successfully.",
    ))
}
