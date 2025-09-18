use axum::{routing::post, Json, Router};

use crate::errors::Error;
use crate::routes::user::dtos::{request::RegisterRequestDto, response::RegisterResponseDto};

pub fn create_route() -> Router {
    Router::new().route("/register", post(register))
}

pub async fn register(
    Json(_request): Json<RegisterRequestDto>,
) -> Result<Json<RegisterResponseDto>, Error> {
    Ok(Json(RegisterResponseDto {}))
}
