use std::sync::Arc;

use crate::{config::Config, services::user_service::UserService};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub user_service: UserService,
}

impl AppState {
    pub fn new(config: Config, user_service: UserService) -> Self {
        Self {
            config: Arc::new(config),
            user_service,
        }
    }
}
