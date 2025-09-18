use axum::{response::IntoResponse, routing::get, Router};

pub fn create_route() -> Router {
    Router::new().route("/", get(health_check))
}

pub async fn health_check() -> impl IntoResponse {
    "OK"
}
