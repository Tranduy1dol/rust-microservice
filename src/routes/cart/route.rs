use axum::{
    extract::{Path, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use rust_decimal::Decimal;
use validator::Validate;

use super::dtos::{AddItemRequestDto, CartItemResponseDto, CartResponseDto, UpdateItemRequestDto};
use crate::app::AppState;
use crate::{auth::jwt::Auth, errors::Error};

pub fn create_route() -> Router<AppState> {
    Router::new()
        .route("/", get(get_cart))
        .route("/item", post(add_item))
        .route("/item/:product_id", patch(update_item))
        .route("/item/:product_id", delete(remove_item))
}

async fn get_cart(
    State(state): State<AppState>,
    Auth(claim): Auth,
) -> Result<Json<CartResponseDto>, Error> {
    let (cart, items_with_products) = state
        .cart_repo
        .get_cart_by_user_id(claim.sub)
        .await?
        .ok_or(Error::not_found("Cart not found".to_string()))?;

    let mut total_amount = Decimal::ZERO;
    let items: Vec<CartItemResponseDto> = items_with_products
        .into_iter()
        .map(|(item, product)| {
            let total_price = product.price * Decimal::from(item.quantity);
            total_amount += total_price;
            CartItemResponseDto {
                product_id: item.product_id,
                quantity: item.quantity,
                product_name: product.name,
                price_per_unit: product.price,
                total_price,
            }
        })
        .collect();

    Ok(Json(CartResponseDto {
        id: cart.id,
        user_id: cart.user_id,
        items,
        total_amount,
    }))
}

async fn add_item(
    State(state): State<AppState>,
    Auth(claim): Auth,
    Json(request): Json<AddItemRequestDto>,
) -> Result<Json<CartResponseDto>, Error> {
    request.validate().map_err(Error::from)?;

    let product = state
        .product_repo
        .get_product_by_id(request.product_id)
        .await?;

    if product.stock_quantity < request.quantity {
        return Err(Error::bad_request("Not enough stock".to_string()));
    }

    state
        .cart_repo
        .set_item_quantity(claim.sub, request.product_id, request.quantity)
        .await?;

    get_cart(State(state), Auth(claim)).await
}

/// Cập nhật số lượng
async fn update_item(
    State(state): State<AppState>,
    Auth(claim): Auth,
    Path(product_id): Path<i64>,
    Json(request): Json<UpdateItemRequestDto>,
) -> Result<Json<CartResponseDto>, Error> {
    request.validate().map_err(Error::from)?;

    let product = state.product_repo.get_product_by_id(product_id).await?;

    if product.stock_quantity < request.quantity {
        return Err(Error::bad_request("Not enough stock".to_string()));
    }

    state
        .cart_repo
        .set_item_quantity(claim.sub, product_id, request.quantity)
        .await?;

    get_cart(State(state), Auth(claim)).await
}

async fn remove_item(
    State(state): State<AppState>,
    Auth(claim): Auth,
    Path(product_id): Path<i64>,
) -> Result<Json<CartResponseDto>, Error> {
    state.cart_repo.remove_item(claim.sub, product_id).await?;

    get_cart(State(state), Auth(claim)).await
}
