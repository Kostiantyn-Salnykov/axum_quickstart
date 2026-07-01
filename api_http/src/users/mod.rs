pub mod get;
pub mod search;

use crate::state::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().merge(get::router()).merge(search::router())
}
