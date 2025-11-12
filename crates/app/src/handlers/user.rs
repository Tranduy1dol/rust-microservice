use app_core::dto::user_dto::{LoginDto, RegisterUserDto};
use axum::{
    Router, extract::State, http::StatusCode, response::IntoResponse, response::Json, routing::post,
};
use serde_json::json;

use crate::state::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(state)
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserDto>,
) -> impl IntoResponse {
    match state.user_service.register(payload).await {
        Ok(user) => (StatusCode::CREATED, Json(json!({"userId": user.id}))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginDto>,
) -> impl IntoResponse {
    match state.user_service.login(payload).await {
        Ok(token) => (StatusCode::OK, Json(json!({"token": token}))).into_response(),
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}
