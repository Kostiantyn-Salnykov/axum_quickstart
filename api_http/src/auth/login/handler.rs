use crate::auth::login::request::LoginRequest;
use crate::auth::login::response::LoginResponse;
use crate::errors::AppError;
use crate::responses::JsendResponse;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn login(
    State(state): State<AppState>,
    payload: Result<Json<LoginRequest>, JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Json(payload) = payload.map_err(AppError::from_json_rejection)?;
    let result = state
        .auth_login
        .login(payload.email, payload.password)
        .await?;

    Ok(JsendResponse::success(
        StatusCode::OK,
        LoginResponse::from(result),
        "User logged in successfully.",
    ))
}
