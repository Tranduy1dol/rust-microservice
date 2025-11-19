use app_core::dto::cart_dto::{AddCartItemDto, CartDto};
use app_core::error::Error;
use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    routing::{delete, get, post},
};
use validator::Validate;

use crate::state::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_cart))
        .route("/items", post(add_item))
        .route("/items/:product_id", delete(remove_item))
        .with_state(state)
}

pub async fn get_cart(
    State(state): State<AppState>,
    Extension(user_id): Extension<i64>,
) -> Result<Json<CartDto>, Error> {
    let cart = state.cart_service.get_cart(user_id).await?;
    Ok(Json(cart))
}

pub async fn add_item(
    State(state): State<AppState>,
    Extension(user_id): Extension<i64>,
    Json(dto): Json<AddCartItemDto>,
) -> Result<Json<CartDto>, Error> {
    dto.validate()?;
    let cart = state
        .cart_service
        .add_item(user_id, dto.product_id, dto.quantity)
        .await?;
    Ok(Json(cart))
}

pub async fn remove_item(
    State(state): State<AppState>,
    Extension(user_id): Extension<i64>,
    Path(product_id): Path<i64>,
) -> Result<Json<CartDto>, Error> {
    let cart = state.cart_service.remove_item(user_id, product_id).await?;
    Ok(Json(cart))
}
