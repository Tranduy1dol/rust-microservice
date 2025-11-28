use app_core::{
    dto::cart_dto::{AddCartItemDto, CartDto},
    error::Error,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post},
};
use validator::Validate;

use crate::{extractors::auth::JwtAuth, state::AppState};

/// Creates an Axum `Router` configured for cart operations and attaches the provided application state.
///
/// The returned router exposes three endpoints:
/// - `GET /` — fetch the current user's cart
/// - `POST /items` — add an item to the current user's cart
/// - `DELETE /items/:product_id` — remove an item from the current user's cart
///
/// # Examples
///
/// ```
/// // Obtain or construct your `AppState`, then create the router:
/// // let state = /* your AppState instance */;
/// // let router = routes(state);
/// ```
pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_cart))
        .route("/items", post(add_item))
        .route("/items/{product_id}", delete(remove_item))
        .with_state(state)
}

/// Retrieves the authenticated user's cart and returns it as JSON.
///
/// # Examples
///
/// ```ignore
/// use axum::extract::State;
/// use axum::response::Json;
/// use your_crate::state::AppState;
/// use your_crate::auth::{JwtAuth, Claims};
/// use your_crate::handlers::cart::get_cart;
/// use your_crate::dto::CartDto;
///
/// let state: AppState = todo!();
/// let claims = Claims { sub: 42, ..Default::default() };
///
/// let response: Result<Json<CartDto>, _> = tokio::runtime::Runtime::new()
///     .unwrap()
///     .block_on(async { get_cart(State(state), JwtAuth(claims)).await });
/// ```
pub async fn get_cart(
    State(state): State<AppState>,
    JwtAuth(claims): JwtAuth,
) -> Result<Json<CartDto>, Error> {
    let cart = state.cart_service.get_cart(claims.sub).await?;
    Ok(Json(cart))
}

/// Add the specified product and quantity to the authenticated user's cart.
///
/// Validates the provided `AddCartItemDto` and updates the user's cart, returning the cart's new state.
///
/// # Returns
///
/// The updated `CartDto`.
///
/// # Examples
///
/// ```ignore
/// # use axum::{extract::{State, Json}};
/// # use crate::state::AppState;
/// # use crate::auth::JwtAuth;
/// # use crate::handlers::cart::{add_item, AddCartItemDto, CartDto};
/// # use crate::auth::Claims;
/// # async fn example() {
/// let state: AppState = /* initialize app state */ todo!();
/// let claims = Claims { sub: 42, ..Default::default() };
/// let dto = AddCartItemDto { product_id: 7, quantity: 2 };
/// let result = add_item(State(state), JwtAuth(claims), Json(dto)).await;
/// # }
/// ```
pub async fn add_item(
    State(state): State<AppState>,
    JwtAuth(claims): JwtAuth,
    Json(dto): Json<AddCartItemDto>,
) -> Result<Json<CartDto>, Error> {
    dto.validate()?;
    let cart = state
        .cart_service
        .add_item(claims.sub, dto.product_id, dto.quantity)
        .await?;
    Ok(Json(cart))
}

/// Remove a product from the authenticated user's cart and return the updated cart.
///
/// # Returns
///
/// The updated cart as a `CartDto`.
///
/// # Examples
///
/// ```ignore
/// #[tokio::test]
/// async fn remove_item_example() {
///     // `state`, `JwtAuth(claims)`, and middleware-provided values are supplied by the application in real use.
///     // This example demonstrates the call shape only.
///     let state = /* AppState instance */ todo!();
///     let claims = /* JwtAuth claims with `sub` set to user id */ todo!();
///     let result = super::remove_item(
///         axum::extract::State(state),
///         JwtAuth(claims),
///         axum::extract::Path(7i64),
///     ).await;
///     let _updated_cart = result.unwrap();
/// }
/// ```
pub async fn remove_item(
    State(state): State<AppState>,
    JwtAuth(claims): JwtAuth,
    Path(product_id): Path<i64>,
) -> Result<Json<CartDto>, Error> {
    let cart = state
        .cart_service
        .remove_item(claims.sub, product_id)
        .await?;
    Ok(Json(cart))
}
