use crate::{
    dto::user::{CreateUserRequest, UserResponse},
    error::AppError,
    repository::user_repository::UserRepository,
};
use sqlx::Error;
use tracing::{error, info, warn};

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
            .map_err(|e| {
                error!("error retrieving user id={id}: {e}");
                AppError::Internal
            })?
            .ok_or(AppError::NotFound)?;
        info!("user retrieved id={id}");
        Ok(user.into())
    }

    pub async fn create_user(&self, req: CreateUserRequest) -> Result<UserResponse, AppError> {
        if req.email.trim().is_empty() {
            warn!("attempted to create user with empty email");
            return Err(AppError::BadRequest("email required".into()));
        }
        let email = req.email.clone();
        let user = self
            .repo
            .insert(req.name, req.email)
            .await
            .map_err(|e| {
                let is_dup = matches!(
                    &e,
                    Error::Database(db_err) if db_err
                        .code()
                        .map_or(false, |code| code == "23505")
                );
                if is_dup {
                    warn!("duplicate email: {}", email);
                    return AppError::Conflict("email already exists".into());
                }
                error!("error creating user email={}: {}", email, e);
                AppError::Internal
            })?;
        info!("user created id={} email={}", user.id, user.email);
        Ok(user.into())
    }
}