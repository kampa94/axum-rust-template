use std::sync::Arc;

use crate::{
    config::Config,
    services::{post_service::PostService, user_service::UserService},
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub user_service: UserService,
    pub post_service: PostService,
}

impl AppState {
    pub fn new(config: Config, user_service: UserService, post_service: PostService) -> Self {
        Self {
            config: Arc::new(config),
            user_service,
            post_service,
        }
    }
}
