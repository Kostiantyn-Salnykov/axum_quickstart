use crate::auth::logout::response::LogoutResponse;
use crate::errors::AppError;
use crate::middlewares::{VerifiedToken, bearer_token};
use crate::responses::JsendResponse;
use crate::state::AppState;
use axum::extract::{Extension, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;

pub async fn logout(
    State(state): State<AppState>,
    Extension(verified): Extension<VerifiedToken>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let token = bearer_token(&headers)?;
    state
        .auth
        .logout
        .logout(token.to_string(), verified.0.expires_at)
        .await?;

    Ok(JsendResponse::success(
        StatusCode::OK,
        LogoutResponse {},
        "User logged out successfully.",
    ))
}
