use crate::errors::AppError;
use crate::responses::JsendResponse;
use crate::state::AppState;
use crate::users::search::request::UserSearchRequest;
use crate::users::search::response::{UserSearchResponse, stream_line};
use axum::body::Body;
use axum::extract::{Json, State, rejection::JsonRejection};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use std::convert::Infallible;
use tokio_stream::iter;

pub async fn search(
    State(state): State<AppState>,
    payload: Result<Json<UserSearchRequest>, JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Json(payload) = payload.map_err(AppError::from_json_rejection)?;
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
    payload: Result<Json<UserSearchRequest>, JsonRejection>,
) -> Result<Response, AppError> {
    let Json(payload) = payload.map_err(AppError::from_json_rejection)?;
    let (query, projection) = payload.into_query()?;
    let result = state.users.search.search(query).await?;

    let lines = result
        .items
        .into_iter()
        .map(|item| {
            stream_line(item, &projection)
                .map(|line| format!("{line}\n"))
                .map_err(|error| AppError::Internal(error.into()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let stream = iter(
        lines
            .into_iter()
            .map(|line| Ok::<Bytes, Infallible>(Bytes::from(line))),
    );
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
