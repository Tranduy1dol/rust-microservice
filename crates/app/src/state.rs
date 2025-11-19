use std::sync::Arc;

use app_core::service::{
    cart_service::CartService, checkout_service::CheckoutService, product_service::ProductService,
    user_service::UserService,
};

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
    pub product_service: Arc<ProductService>,
    pub cart_service: Arc<CartService>,
    pub checkout_service: Arc<CheckoutService>,
}
