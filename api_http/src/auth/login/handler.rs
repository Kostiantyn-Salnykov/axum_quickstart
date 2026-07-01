use crate::auth::login::request::LoginRequest;
use crate::auth::login::response::LoginResponse;
use crate::content::ContentBody;
use crate::errors::AppError;
use crate::responses::JsendResponse;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn login(
    State(state): State<AppState>,
    payload: Result<ContentBody<LoginRequest>, crate::content::ContentBodyRejection>,
) -> Result<impl IntoResponse, AppError> {
    let ContentBody(payload) = payload.map_err(AppError::from_content_body_rejection)?;
    let result = state
        .auth
        .login
        .login(payload.email, payload.password)
        .await?;

    Ok(JsendResponse::success(
        StatusCode::OK,
        LoginResponse::from(result),
        "User logged in successfully.",
    ))
}
