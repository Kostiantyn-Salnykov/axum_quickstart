pub mod login;
pub mod logout;
pub mod refresh;
pub mod register;

use crate::state::AppState;
use axum::Router;

pub fn public_router() -> Router<AppState> {
    Router::new()
        .merge(register::router())
        .merge(login::router())
        .merge(refresh::router())
}

pub fn protected_router() -> Router<AppState> {
    Router::new().merge(logout::router())
}

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(public_router())
        .merge(protected_router())
}
