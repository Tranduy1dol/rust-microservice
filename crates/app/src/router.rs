use axum::Router;

use super::{handlers, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new().nest("/api/v1", api_routes(state))
}

fn api_routes(state: AppState) -> Router {
    Router::new().nest("/users", handlers::user::routes(state))
}
