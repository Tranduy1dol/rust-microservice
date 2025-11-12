use std::sync::Arc;

use app_core::service::user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
}
