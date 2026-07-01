use crate::errors::AppError;
use axum::Json;
use axum::body::{Body, to_bytes};
use axum::extract::{FromRequest, Request, rejection::JsonRejection};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Display;
use tokio::task_local;

const JSON_MEDIA_TYPE: &str = "application/json";
const MSGPACK_MEDIA_TYPE: &str = "application/msgpack";
const MSGPACK_MEDIA_TYPE_ALIAS: &str = "application/x-msgpack";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseFormat {
    Json,
    MsgPack,
}

task_local! {
    static RESPONSE_FORMAT: ResponseFormat;
}

pub async fn negotiate_response_format(request: Request, next: Next) -> Response {
    let format = response_format_from_headers(request.headers());

    RESPONSE_FORMAT
        .scope(format, async move { next.run(request).await })
        .await
}

pub fn current_response_format() -> ResponseFormat {
    RESPONSE_FORMAT
        .try_with(|format| *format)
        .unwrap_or(ResponseFormat::Json)
}

pub fn response_format_from_headers(headers: &HeaderMap) -> ResponseFormat {
    let accept = headers
        .get(header::ACCEPT)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_ascii_lowercase();

    if accepts_msgpack(&accept) {
        ResponseFormat::MsgPack
    } else {
        ResponseFormat::Json
    }
}

fn accepts_msgpack(accept: &str) -> bool {
    accept.contains(MSGPACK_MEDIA_TYPE) || accept.contains(MSGPACK_MEDIA_TYPE_ALIAS)
}

fn content_type_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(|value| {
            value
                .split(';')
                .next()
                .unwrap_or_default()
                .trim()
                .to_ascii_lowercase()
        })
}

fn is_json_content_type(content_type: &str) -> bool {
    content_type == JSON_MEDIA_TYPE || content_type.ends_with("+json")
}

fn is_msgpack_content_type(content_type: &str) -> bool {
    content_type == MSGPACK_MEDIA_TYPE
        || content_type == MSGPACK_MEDIA_TYPE_ALIAS
        || content_type.ends_with("+msgpack")
}

#[derive(Debug)]
pub enum ContentBodyRejection {
    MissingContentType,
    UnsupportedContentType(String),
    Json(JsonRejection),
    MsgPack(String),
    Body(String),
}

impl Display for ContentBodyRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingContentType => write!(
                f,
                "Missing `content-type: application/json` or `application/msgpack` header."
            ),
            Self::UnsupportedContentType(content_type) => {
                write!(f, "Unsupported content type `{content_type}`.")
            }
            Self::Json(rejection) => write!(f, "{}", rejection.body_text()),
            Self::MsgPack(message) | Self::Body(message) => write!(f, "{message}"),
        }
    }
}

impl ContentBodyRejection {
    pub fn into_app_error(self) -> AppError {
        match self {
            Self::MissingContentType => AppError::BadRequest(
                "Missing `content-type: application/json` or `application/msgpack` header."
                    .to_string(),
            ),
            Self::UnsupportedContentType(content_type) => {
                AppError::BadRequest(format!("Unsupported content type `{content_type}`."))
            }
            Self::Body(message) => AppError::BadRequest(message),
            Self::Json(rejection) => AppError::from_json_rejection(rejection),
            Self::MsgPack(message) => AppError::Validation(message),
        }
    }
}

impl IntoResponse for ContentBodyRejection {
    fn into_response(self) -> Response {
        self.into_app_error().into_response()
    }
}

pub struct ContentBody<T>(pub T);

impl<S, T> FromRequest<S> for ContentBody<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = ContentBodyRejection;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let content_type = content_type_from_headers(request.headers())
            .ok_or(ContentBodyRejection::MissingContentType)?;

        if is_json_content_type(&content_type) {
            let Json(value) = Json::<T>::from_request(request, state)
                .await
                .map_err(ContentBodyRejection::Json)?;
            return Ok(Self(value));
        }

        if is_msgpack_content_type(&content_type) {
            let bytes = to_bytes(request.into_body(), usize::MAX)
                .await
                .map_err(|error| ContentBodyRejection::Body(error.to_string()))?;
            let value = rmp_serde::from_slice::<T>(&bytes)
                .map_err(|error| ContentBodyRejection::MsgPack(error.to_string()))?;
            return Ok(Self(value));
        }

        Err(ContentBodyRejection::UnsupportedContentType(content_type))
    }
}

pub fn serialize_response<T: Serialize>(status: StatusCode, body: T) -> Response {
    match current_response_format() {
        ResponseFormat::Json => (status, Json(body)).into_response(),
        ResponseFormat::MsgPack => serialize_msgpack_response(status, body),
    }
}

fn serialize_msgpack_response<T: Serialize>(status: StatusCode, body: T) -> Response {
    match rmp_serde::to_vec_named(&body) {
        Ok(bytes) => Response::builder()
            .status(status)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static(MSGPACK_MEDIA_TYPE),
            )
            .body(Body::from(bytes))
            .unwrap_or_else(|error| internal_server_error_response(error.to_string())),
        Err(error) => internal_server_error_response(error.to_string()),
    }
}

fn internal_server_error_response(message: String) -> Response {
    tracing::error!(error = %message, "Failed to serialize msgpack response.");
    match current_response_format() {
        ResponseFormat::Json => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(fallback_error_body()),
        )
            .into_response(),
        ResponseFormat::MsgPack => match rmp_serde::to_vec_named(&fallback_error_body()) {
            Ok(bytes) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(MSGPACK_MEDIA_TYPE),
                )
                .body(Body::from(bytes))
                .unwrap_or_else(|error| {
                    tracing::error!(error = %error, "Failed to build fallback msgpack response.");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(fallback_error_body()),
                    )
                        .into_response()
                }),
            Err(error) => {
                tracing::error!(error = %error, "Failed to serialize fallback msgpack response.");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(fallback_error_body()),
                )
                    .into_response()
            }
        },
    }
}

fn fallback_error_body() -> crate::responses::JsendResponse<()> {
    crate::responses::JsendResponse::<()> {
        status: crate::enums::JsendStatus::Error,
        code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        message: Some("Internal server error".to_string()),
        data: None,
    }
}
