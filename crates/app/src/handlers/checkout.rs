use app_core::error::Error;
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde_json::json;

use crate::{extractors::auth::JwtAuth, state::AppState};

/// Creates an Axum router that mounts the checkout handler at the POST "/" endpoint and attaches the provided application state.
///
/// # Examples
///
/// ```ignore
/// use axum::Router;
/// use crate::handlers::checkout::routes;
/// use crate::state::AppState;
///
/// // Build your application state somehow (example uses a placeholder)
/// let state: AppState = /* build AppState */ unimplemented!();
/// let router: Router = routes(state);
/// // `router` now has a POST "/" route wired to the checkout handler with `state` available via extractors.
/// ```
pub fn routes(state: AppState) -> Router {
    Router::new().route("/", post(checkout)).with_state(state)
}

/// Create an order for the authenticated user and respond with the created order ID.
///
/// On success returns an HTTP 201 Created status and a JSON body containing the created
/// `orderId`.
///
/// # Examples
///
/// ```ignore
/// use axum::http::StatusCode;
/// use serde_json::json;
/// use axum::Json;
///
/// // Example of expected response shape (handler invocation omitted)
/// let resp: (StatusCode, Json<serde_json::Value>) = (StatusCode::CREATED, Json(json!({"orderId": 42})));
/// assert_eq!(resp.0, StatusCode::CREATED);
/// assert_eq!(resp.1.0["orderId"], json!(42));
/// ```
pub async fn checkout(
    State(state): State<AppState>,
    JwtAuth(claims): JwtAuth,
) -> Result<(StatusCode, Json<serde_json::Value>), Error> {
    let order = state.checkout_service.checkout(claims.sub).await?;
    Ok((StatusCode::CREATED, Json(json!({"orderId": order.id}))))
}