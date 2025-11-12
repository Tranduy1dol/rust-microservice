use app_core::dto::user_dto::{LoginDto, RegisterUserDto};
use axum::{
    Router, extract::State, http::StatusCode, response::IntoResponse, response::Json, routing::post,
};
use serde_json::json;

use crate::state::AppState;

/// Builds the user-related HTTP router and attaches the provided application state.
///
/// Mounts POST /register to the `register` handler and POST /login to the `login` handler,
/// then returns the router configured with the shared `AppState`.
///
/// # Examples
///
/// ```
/// use crate::state::AppState;
/// use app::handlers::user::routes;
///
/// // assume `state` is created elsewhere
/// let state: AppState = /* ... */;
/// let router = routes(state);
/// ```
pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(state)
}

/// Handle user registration requests.
///
/// On success responds with HTTP 201 Created with a JSON body `{"userId": <id>}`.
/// On failure responds with HTTP 400 Bad Request with the error message as plain text.
///
/// # Examples
///
/// ```
/// // Successful registration -> 201 Created with JSON {"userId": 42}
/// // Failed registration -> 400 Bad Request with error string
/// ```
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserDto>,
) -> impl IntoResponse {
    match state.user_service.register(payload).await {
        Ok(user) => (StatusCode::CREATED, Json(json!({"userId": user.id}))).into_response(),
        Err(e) => {
            // Log the detailed error for debugging
            tracing::error!("Registration failed: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Registration failed"})),
            )
                .into_response()
        }
    }
}

/// Authenticates a user and returns an HTTP response containing a bearer token on success.
///
/// On success returns HTTP 200 with JSON body `{"token": "<token>"}`. On failure returns HTTP 401 with the error message as plain text.
///
/// # Examples
///
/// ```
/// use axum::Json;
/// // Construct AppState and LoginDto appropriately in real code.
/// # async fn example() {
/// # let state = /* AppState */ todo!();
/// # let dto = /* LoginDto */ todo!();
/// let response = crate::handlers::user::login(axum::extract::State(state), Json(dto)).await;
/// # }
/// ```
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginDto>,
) -> impl IntoResponse {
    match state.user_service.login(payload).await {
        Ok(token) => (StatusCode::OK, Json(json!({"token": token}))).into_response(),
        Err(e) => {
            // Log the detailed error for debugging
            tracing::error!("Login failed: {}", e);
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid credentials"})),
            )
                .into_response()
        }
    }
}
