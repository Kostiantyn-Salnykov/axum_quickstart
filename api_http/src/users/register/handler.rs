use crate::errors::AppError;
use crate::responses::JsendResponse;
use crate::state::AppState;
use crate::users::register::request::RegisterUserRequest;
use crate::users::register::response::RegisterUserResponse;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn register_user(
    State(state): State<AppState>,
    payload: Result<Json<RegisterUserRequest>, JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Json(payload) = payload.map_err(AppError::from_json_rejection)?;
    let user = state
        .register_user
        .register(
            payload.email,
            payload.password,
            payload.first_name,
            payload.last_name,
        )
        .await?;

    Ok(JsendResponse::success(
        StatusCode::CREATED,
        RegisterUserResponse::from(user),
        "User registered successfully.",
    ))
}
