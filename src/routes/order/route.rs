use axum::{routing::post, Json, Router};

use crate::errors::Error;
use crate::routes::order::dtos::{
    request::CreateOrderRequestDto, response::CreateOrderResponseDto,
};

pub fn create_route() -> Router {
    Router::new().route("/", post(create_order))
}

pub async fn create_order(
    Json(_request): Json<CreateOrderRequestDto>,
) -> Result<Json<CreateOrderResponseDto>, Error> {
    Ok(Json(CreateOrderResponseDto {}))
}
