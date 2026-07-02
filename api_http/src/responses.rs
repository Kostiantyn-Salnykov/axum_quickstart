use crate::enums::JsendStatus;
use crate::errors::AppError;
use application::rate_limit::policy::RateLimitInfo;
use axum::http::StatusCode;
use axum::http::{HeaderValue, header};
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub struct JsendResponse<T: Serialize> {
    pub status: JsendStatus,
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> JsendResponse<T> {
    pub fn success(code: StatusCode, data: T, message: impl Into<String>) -> Self {
        Self {
            status: JsendStatus::Success,
            code: code.as_u16(),
            message: Some(message.into()),
            data: Some(data),
        }
    }
}

impl<T: Serialize> IntoResponse for JsendResponse<T> {
    fn into_response(self) -> Response {
        let http_status =
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        crate::content::serialize_response(http_status, self)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (http_status, jsend_status, message, rate_limit_info) = match &self {
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                JsendStatus::Fail,
                msg.clone(),
                None,
            ),
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, JsendStatus::Fail, msg.clone(), None)
            }
            AppError::Validation(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                JsendStatus::Fail,
                msg.clone(),
                None,
            ),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, JsendStatus::Fail, msg.clone(), None),
            AppError::RateLimited { info, message } => (
                StatusCode::TOO_MANY_REQUESTS,
                JsendStatus::Fail,
                message.clone(),
                Some(*info),
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                JsendStatus::Fail,
                "Unauthorized".to_string(),
                None,
            ),
            AppError::Internal(error) => {
                tracing::error!(error = ?error, "Unhandled internal application error.");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    JsendStatus::Error,
                    "Internal server error".to_string(),
                    None,
                )
            }
        };

        let body = JsendResponse::<()> {
            status: jsend_status,
            code: http_status.as_u16(),
            message: Some(message),
            data: None,
        };

        let response = crate::content::serialize_response(http_status, body);
        if let Some(info) = rate_limit_info {
            with_rate_limit_headers(response, info, true)
        } else {
            response
        }
    }
}

pub fn with_rate_limit_headers(
    mut response: Response,
    info: RateLimitInfo,
    include_retry_after: bool,
) -> Response {
    let headers = response.headers_mut();
    headers.insert(
        header::HeaderName::from_static("ratelimit-limit"),
        HeaderValue::from_str(&info.limit.to_string()).unwrap_or(HeaderValue::from_static("0")),
    );
    headers.insert(
        header::HeaderName::from_static("ratelimit-remaining"),
        HeaderValue::from_str(&info.remaining.to_string()).unwrap_or(HeaderValue::from_static("0")),
    );
    headers.insert(
        header::HeaderName::from_static("ratelimit-reset"),
        HeaderValue::from_str(&info.reset_after_seconds.to_string())
            .unwrap_or(HeaderValue::from_static("0")),
    );
    if include_retry_after {
        headers.insert(
            header::RETRY_AFTER,
            HeaderValue::from_str(&info.reset_after_seconds.to_string())
                .unwrap_or(HeaderValue::from_static("0")),
        );
    }

    response
}
