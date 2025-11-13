use std::sync::Arc;

use app_core::service::{product_service::ProductService, user_service::UserService};

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
    pub product_service: Arc<ProductService>,
}
