use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{
    dto::post::{CreatePostRequest, PostResponse},
    error::AppError,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/{user_id}/posts", get(list_posts).post(create_post))
}

async fn list_posts(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<PostResponse>>, AppError> {
    state.post_service.list_by_user(user_id).await.map(Json)
}

async fn create_post(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
    Json(req): Json<CreatePostRequest>,
) -> Result<Json<PostResponse>, AppError> {
    state.post_service.create(user_id, req).await.map(Json)
}
