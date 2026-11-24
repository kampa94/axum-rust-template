use serde::{Deserialize, Serialize};

use crate::models::post::Post;

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct PostResponse {
    pub id: i64,
    pub title: String,
    pub body: String,
}

impl From<Post> for PostResponse {
    fn from(p: Post) -> Self {
        Self { id: p.id, title: p.title, body: p.body }
    }
}
