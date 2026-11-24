use crate::{
    dto::post::{CreatePostRequest, PostResponse},
    error::AppError,
    repository::post_repository::PostRepository,
};

#[derive(Clone)]
pub struct PostService {
    repo: PostRepository,
}

impl PostService {
    pub fn new(repo: PostRepository) -> Self {
        Self { repo }
    }

    pub async fn list_by_user(&self, user_id: i64) -> Result<Vec<PostResponse>, AppError> {
        let posts = self.repo.find_by_user(user_id).await.map_err(|_| AppError::Internal)?;
        Ok(posts.into_iter().map(Into::into).collect())
    }

    pub async fn create(&self, user_id: i64, req: CreatePostRequest) -> Result<PostResponse, AppError> {
        let post = self
            .repo
            .insert(user_id, req.title, req.body)
            .await
            .map_err(|_| AppError::Internal)?;
        Ok(post.into())
    }
}
