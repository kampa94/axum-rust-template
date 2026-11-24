use axum::Router;

use crate::state::AppState;

pub mod posts;
pub mod users;

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/users", users::router())
        .merge(posts::router())
        .with_state(state)
}