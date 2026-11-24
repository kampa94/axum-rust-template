#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub body: String,
}
