use crate::auth::register::request::RegisterRequest;
use crate::auth::register::response::RegisterResponse;
use crate::content::ContentBody;
use crate::errors::AppError;
use crate::responses::JsendResponse;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn register(
    State(state): State<AppState>,
    payload: Result<ContentBody<RegisterRequest>, crate::content::ContentBodyRejection>,
) -> Result<impl IntoResponse, AppError> {
    let ContentBody(payload) = payload.map_err(AppError::from_content_body_rejection)?;
    let result = state
        .auth
        .register
        .register(
            payload.email,
            payload.phone,
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
