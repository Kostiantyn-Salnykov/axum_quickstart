use axum::http::HeaderName;
use axum::{Router, http, routing::get};
use tower::ServiceBuilder;
use tower_http::request_id::{PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use tracing::Span;

mod enums;
mod errors;
pub mod handlers;
mod middlewares;
pub mod schemas;
pub mod state;

use crate::middlewares::request_id::MakeRequestUuid;
use handlers::health_check::health_check;
use state::AppState;

const REQUEST_ID_HEADER: &str = "x-request-id";

pub fn create_router(state: AppState) -> Router {
    let request_id_header = HeaderName::from_static(REQUEST_ID_HEADER);

    Router::new()
        .route("/health_check/", get(health_check))
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(
                    request_id_header.clone(),
                    MakeRequestUuid,
                ))
                .layer({
                    let request_id_header = request_id_header.clone();

                    TraceLayer::new_for_http()
                        .make_span_with(move |request: &http::Request<_>| {
                            let request_id = request
                                .headers()
                                .get(&request_id_header)
                                .and_then(|v| v.to_str().ok())
                                .unwrap_or("unknown");

                            tracing::debug_span!(
                                "http-request",
                                method = %request.method(),
                                path = %request.uri().path(),
                                request_id = %request_id,
                                status_code = tracing::field::Empty,
                            )
                        })
                        .on_response(
                            |response: &http::Response<_>,
                             latency: std::time::Duration,
                             span: &Span| {
                                span.record("status_code", response.status().as_u16());
                                tracing::debug!(latency_ms = latency.as_millis(), "response");
                            },
                        )
                        .on_failure(|error, latency: std::time::Duration, _span: &Span| {
                            tracing::error!(
                                error = %error,
                                latency_ms = latency.as_millis(),
                                "request failed"
                            );
                        })
                })
                .layer(PropagateRequestIdLayer::new(request_id_header)),
        )
        .with_state(state)
}
