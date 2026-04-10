pub mod login;
pub mod refresh;
pub mod register;

use crate::state::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(register::router())
        .merge(login::router())
        .merge(refresh::router())
}
