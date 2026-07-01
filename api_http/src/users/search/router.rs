use crate::state::AppState;
use crate::users::search::handler::{search, search_stream};
use axum::{Router, routing::post};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/search/", post(search))
        .route("/users/search/stream/", post(search_stream))
}
