use crate::errors::AppError;
use crate::state::AppState;
use application::auth::token_manager_port::{TokenAudience, TokenPayload};
use axum::extract::Request;
use axum::http;
use axum::middleware::Next;
use axum::response::Response;
use tower_http::request_id::{MakeRequestId, RequestId};
use uuid::Uuid;

#[derive(Clone)]
pub struct MakeRequestUuid;

#[derive(Clone)]
pub struct VerifiedToken(pub TokenPayload);

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _request: &http::Request<B>) -> Option<RequestId> {
        let id = Uuid::now_v7().to_string();
        let header_value = id.parse().ok()?;
        Some(RequestId::new(header_value))
    }
}

pub async fn require_auth(
    axum::extract::State(state): axum::extract::State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = bearer_token(request.headers())?;
    tracing::debug!(
        component = "Middleware",
        method = "require_auth",
        token = token,
        "Verifying bearer token."
    );
    let payload = state.auth.token_manager.verify(token).await?;
    if payload.audience != TokenAudience::Access {
        tracing::warn!("Request rejected: invalid or missing access token.");
        return Err(AppError::Unauthorized);
    }
    request.extensions_mut().insert(VerifiedToken(payload));
    Ok(next.run(request).await)
}

pub fn bearer_token(headers: &http::HeaderMap) -> Result<&str, AppError> {
    let header = headers
        .get(http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    header
        .strip_prefix("Bearer ")
        .or_else(|| header.strip_prefix("bearer "))
        .ok_or(AppError::Unauthorized)
}
