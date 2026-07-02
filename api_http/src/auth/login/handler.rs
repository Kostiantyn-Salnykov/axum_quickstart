use crate::auth::login::LOGIN_RATE_LIMIT_POLICIES;
use crate::auth::login::request::LoginRequest;
use crate::auth::login::response::LoginResponse;
use crate::content::ContentBody;
use crate::errors::AppError;
use crate::responses::{JsendResponse, with_rate_limit_headers};
use crate::state::AppState;
use application::rate_limit::check_all;
use axum::extract::ConnectInfo;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::net::SocketAddr;

pub async fn login(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    payload: Result<ContentBody<LoginRequest>, crate::content::ContentBodyRejection>,
) -> Result<impl IntoResponse, AppError> {
    let ContentBody(payload) = payload.map_err(AppError::from_content_body_rejection)?;
    let rate_limit_info = check_all(
        state.rate_limiter.as_ref(),
        "auth:login",
        &addr.ip().to_string(),
        LOGIN_RATE_LIMIT_POLICIES,
    )
    .await?;
    let result = state
        .auth
        .login
        .login(payload.email, payload.password)
        .await?;

    let response = JsendResponse::success(
        StatusCode::OK,
        LoginResponse::from(result),
        "User logged in successfully.",
    )
    .into_response();

    Ok(with_rate_limit_headers(response, rate_limit_info, false))
}
