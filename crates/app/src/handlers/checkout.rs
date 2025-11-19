use axum::{Extension, Json, Router, extract::State, http::StatusCode, routing::post};
use serde_json::json;

use crate::state::AppState;
use app_core::error::Error;

pub fn routes(state: AppState) -> Router {
    Router::new().route("/", post(checkout)).with_state(state)
}

pub async fn checkout(
    State(state): State<AppState>,
    Extension(user_id): Extension<i64>,
) -> Result<(StatusCode, Json<serde_json::Value>), Error> {
    let order = state.checkout_service.checkout(user_id).await?;
    Ok((StatusCode::CREATED, Json(json!({"orderId": order.id}))))
}
