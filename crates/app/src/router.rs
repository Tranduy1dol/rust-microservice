use axum::Router;

use super::{handlers, state::AppState};

/// Builds the application's top-level Router and mounts API routes under `/api/v1`.
///
/// # Examples
///
/// ```ignore
/// use crate::router::create_router;
/// use crate::AppState;
///
/// // construct your AppState as appropriate for your application
/// let state = /* AppState::new(...) */ unimplemented!();
/// let router = create_router(state);
/// ```
pub fn create_router(state: AppState) -> Router {
    Router::new().nest("/api/v1", api_routes(state))
}

/// Constructs a Router mounting API sub-routers for users, products, cart, and checkout.
///
/// Mounts the following paths:
/// - `/users`
/// - `/products`
/// - `/cart`
/// - `/checkout`
///
/// The provided `AppState` is cloned for the `/users`, `/products`, and `/cart` sub-routers;
/// ownership of `state` is moved into the `/checkout` sub-router.
///
/// # Returns
///
/// A `Router` with the above sub-routers nested at their respective paths.
///
/// # Examples
///
/// ```ignore
/// let state = unimplemented!(); // replace with a real `AppState`
/// let router = api_routes(state);
/// ```
fn api_routes(state: AppState) -> Router {
    Router::new()
        .nest("/users", handlers::user::routes(state.clone()))
        .nest("/products", handlers::product::routes(state.clone()))
        .nest("/cart", handlers::cart::routes(state.clone()))
        .nest("/checkout", handlers::checkout::routes(state))
}
