use axum::Router;

use crate::state::AppState;

pub mod users;

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/users", users::router())
        .with_state(state)
}