use std::sync::Arc;

use axum::Router;

use crate::{
    config::DbConfig,
    database::setup_database_connection,
    errors::Error,
    repositories::{
        admin_repository::AdminRepository, cart_repository::CartRepository,
        category_repository::CategoryRepository, order_item_repository::OrderItemRepository,
        order_repository::OrderRepository, product_repository::ProductRepository,
        user_repository::UserRepository,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<UserRepository>,
    pub admin_repo: Arc<AdminRepository>,
    pub product_repo: Arc<ProductRepository>,
    pub category_repo: Arc<CategoryRepository>,
    pub order_repo: Arc<OrderRepository>,
    pub order_item_repo: Arc<OrderItemRepository>,
    pub cart_repo: Arc<CartRepository>,
}

impl AppState {
    pub async fn new(db_url: &str, config: &Option<DbConfig>) -> Result<Self, Error> {
        let database_connection = setup_database_connection(db_url, config).await?;
        Ok(Self {
            user_repo: Arc::new(UserRepository::new(database_connection.clone())),
            admin_repo: Arc::new(AdminRepository::new(database_connection.clone())),
            product_repo: Arc::new(ProductRepository::new(database_connection.clone())),
            category_repo: Arc::new(CategoryRepository::new(database_connection.clone())),
            order_repo: Arc::new(OrderRepository::new(database_connection.clone())),
            order_item_repo: Arc::new(OrderItemRepository::new(database_connection.clone())),
            cart_repo: Arc::new(CartRepository::new(database_connection.clone())),
        })
    }
}

pub async fn create_app(app_state: AppState) -> Router {
    Router::new()
        .nest("/health", crate::routes::health::route::create_route())
        .merge(
            Router::new()
                .nest("/user", crate::routes::user::route::create_route())
                .nest("/product", crate::routes::product::route::create_routes())
                .nest("/category", crate::routes::category::route::create_route())
                .with_state(app_state),
        )
}
