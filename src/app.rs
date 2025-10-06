use axum::Router;

pub async fn create_app() -> Router {
    Router::new()
        .nest("/order", crate::routes::order::route::create_route())
        .nest("/health", crate::routes::health::route::create_route())
        .nest("/user", crate::routes::user::route::create_route())
        .nest("/admin", crate::routes::admin::route::create_route())
        .nest("/product", crate::routes::product::route::create_routes())
}
