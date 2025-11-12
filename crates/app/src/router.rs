use axum::Router;

use super::{handlers, state::AppState};

/// Builds the application's top-level Router and mounts API routes under `/api/v1`.
///
/// # Examples
///
/// ```no_run
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

/// Builds a Router that mounts user-related routes at "/users".
///
/// The returned Router nests the routes produced by `handlers::user::routes(state)`
/// under the "/users" path.
///
/// # Examples
///
/// ```no_run
/// let state = unimplemented!(); // replace with a real `AppState`
/// let router = api_routes(state);
/// ```
fn api_routes(state: AppState) -> Router {
    Router::new().nest("/users", handlers::user::routes(state))
}
