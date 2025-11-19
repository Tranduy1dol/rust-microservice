use app_core::dto::cart_dto::{AddCartItemDto, CartDto};
use app_core::error::Error;
use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    routing::{delete, get, post},
};
use validator::Validate;

use crate::state::AppState;

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
        .route("/items/:product_id", delete(remove_item))
        .with_state(state)
}

/// Fetches the cart belonging to the specified user.
///
/// Returns the user's cart serialized as JSON.
///
/// # Examples
///
/// ```no_run
/// use axum::{Extension, extract::State, Json};
/// use your_crate::state::AppState;
/// use your_crate::handlers::cart::get_cart;
/// use your_crate::dto::CartDto;
///
/// // `state` and `user_id` would be provided by your application runtime.
/// // This demonstrates the callsite shape; do not run as-is.
/// let state: AppState = /* ... */ todo!();
/// let user_id: i64 = 42;
///
/// let response: Result<Json<CartDto>, _> = tokio::runtime::Runtime::new()
///     .unwrap()
///     .block_on(async { get_cart(State(state), Extension(user_id)).await });
/// ```
pub async fn get_cart(
    State(state): State<AppState>,
    Extension(user_id): Extension<i64>,
) -> Result<Json<CartDto>, Error> {
    let cart = state.cart_service.get_cart(user_id).await?;
    Ok(Json(cart))
}

/// Adds a product to the specified user's cart and returns the updated cart.
///
/// Validates the provided `AddCartItemDto` and delegates to the application's cart service
/// to insert or update the item. May return an error if validation fails or the service
/// operation fails.
///
/// # Returns
///
/// The updated `CartDto` on success.
///
/// # Examples
///
/// ```no_run
/// # use axum::{extract::{State, Extension}, Json};
/// # use crate::state::AppState;
/// # use crate::handlers::cart::{add_item, AddCartItemDto, CartDto};
/// # async fn example() {
/// let state: AppState = /* initialize app state */ todo!();
/// let user_id = 42i64;
/// let dto = AddCartItemDto { product_id: 7, quantity: 2 };
/// let result = add_item(State(state), Extension(user_id), Json(dto)).await;
/// # }
/// ```
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

/// Removes a product from the specified user's cart and returns the updated cart.
///
/// # Returns
///
/// The updated cart as a `CartDto`.
///
/// # Examples
///
/// ```no_run
/// #[tokio::test]
/// async fn remove_item_example() {
///     // `state` and middleware-provided `user_id` would be supplied by the application in real use.
///     // This example demonstrates the call shape only.
///     let state = /* AppState instance */ todo!();
///     let result = super::remove_item(
///         axum::extract::State(state),
///         axum::extract::Extension(42i64),
///         axum::extract::Path(7i64),
///     ).await;
///     let _updated_cart = result.unwrap();
/// }
/// ```
pub async fn remove_item(
    State(state): State<AppState>,
    Extension(user_id): Extension<i64>,
    Path(product_id): Path<i64>,
) -> Result<Json<CartDto>, Error> {
    let cart = state.cart_service.remove_item(user_id, product_id).await?;
    Ok(Json(cart))
}