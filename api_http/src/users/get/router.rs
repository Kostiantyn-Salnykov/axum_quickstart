use crate::state::AppState;
use crate::users::get::handler::{get_by_id, get_me};
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/me/", get(get_me))
        .route("/users/{id}/", get(get_by_id))
}
