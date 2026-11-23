use crate::{
    dto::user::{CreateUserRequest, UserResponse},
    error::AppError,
    repository::user_repository::UserRepository,
};

#[derive(Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn get_user(&self, id: i64) -> Result<UserResponse, AppError> {
        let user = self
            .repo
            .find_by_id(id)
            .await
            .map_err(|_| AppError::Internal)?
            .ok_or(AppError::NotFound)?;
        Ok(user.into())
    }

    pub async fn create_user(&self, req: CreateUserRequest) -> Result<UserResponse, AppError> {
        if req.email.trim().is_empty() {
            return Err(AppError::BadRequest("email required".into()));
        }
        let user = self
            .repo
            .insert(req.name, req.email)
            .await
            .map_err(|_| AppError::Internal)?;
        Ok(user.into())
    }
}