use crate::content::ContentBody;
use crate::errors::AppError;
use crate::responses::JsendResponse;
use crate::state::AppState;
use crate::users::search::request::UserSearchRequest;
use crate::users::search::response::{UserSearchResponse, stream_line};
use application::search::query::SearchPagination;
use async_stream::stream;
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;

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
    let (mut query, projection) = payload.into_query()?;
    let stream = stream! {
        loop {
            let result = match state.users.search.search(query.clone()).await {
                Ok(result) => result,
                Err(error) => {
                    yield Err::<Bytes, AppError>(error.into());
                    return;
                }
            };

            for item in result.items {
                let line = match stream_line(item, &projection) {
                    Ok(line) => line,
                    Err(error) => {
                        yield Err::<Bytes, AppError>(AppError::Internal(error.into()));
                        return;
                    }
                };

                yield Ok(Bytes::from(format!("{line}\n")));
            }

            if !result.pagination.has_more {
                return;
            }

            let Some(next_cursor) = result.pagination.next_cursor else {
                tracing::warn!(
                    "Search stream reported more results without a next cursor. Stopping the stream."
                );
                return;
            };

            query.pagination = SearchPagination::Cursor {
                cursor: Some(next_cursor),
                limit: result.pagination.limit,
            };
        }
    };
    let body = Body::from_stream(stream);

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/x-ndjson; charset=utf-8"),
        )
        .header(header::CONNECTION, HeaderValue::from_static("close"))
        .body(body)
        .map_err(|error| AppError::Internal(error.into()))
}
