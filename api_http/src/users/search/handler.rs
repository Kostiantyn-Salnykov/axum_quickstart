use crate::content::ContentBody;
use crate::errors::AppError;
use crate::responses::JsendResponse;
use crate::state::AppState;
use crate::users::search::request::UserSearchRequest;
use crate::users::search::response::{UserSearchResponse, stream_line};
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use tokio_stream::{StreamExt, wrappers::ReceiverStream};

pub async fn search(
    State(state): State<AppState>,
    payload: Result<ContentBody<UserSearchRequest>, crate::content::ContentBodyRejection>,
) -> Result<impl IntoResponse, AppError> {
    let ContentBody(payload) = payload.map_err(AppError::from_content_body_rejection)?;
    let (query, projection) = payload.into_query()?;
    let result = state.users.search.search(query).await?;

    Ok(JsendResponse::success(
        StatusCode::OK,
        UserSearchResponse::from_result(result, &projection),
        "Users fetched successfully.",
    ))
}

pub async fn search_stream(
    State(state): State<AppState>,
    payload: Result<ContentBody<UserSearchRequest>, crate::content::ContentBodyRejection>,
) -> Result<Response, AppError> {
    let ContentBody(payload) = payload.map_err(AppError::from_content_body_rejection)?;
    let (query, projection) = payload.into_query()?;
    let stream =
        ReceiverStream::new(state.users.search.stream(query).await?).map(move |item| match item {
            Ok(item) => stream_line(item, &projection)
                .map(|line| Bytes::from(format!("{line}\n")))
                .map_err(|error| AppError::Internal(error.into())),
            Err(error) => Err(error.into()),
        });

    let body = Body::from_stream(stream);

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/x-ndjson; charset=utf-8"),
        )
        .body(body)
        .map_err(|error| AppError::Internal(error.into()))
}
