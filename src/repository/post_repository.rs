use sqlx::PgPool;

use crate::models::post::Post;

#[derive(Clone)]
pub struct PostRepository {
    pool: PgPool,
}

impl PostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_user(&self, user_id: i64) -> Result<Vec<Post>, sqlx::Error> {
        sqlx::query_as::<_, Post>(
            "SELECT id, user_id, title, body FROM posts WHERE user_id = $1 ORDER BY id",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn insert(&self, user_id: i64, title: String, body: String) -> Result<Post, sqlx::Error> {
        sqlx::query_as::<_, Post>(
            "INSERT INTO posts (user_id, title, body) VALUES ($1, $2, $3)
             RETURNING id, user_id, title, body",
        )
        .bind(user_id)
        .bind(title)
        .bind(body)
        .fetch_one(&self.pool)
        .await
    }
}
