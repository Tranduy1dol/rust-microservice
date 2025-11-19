use app_core::dto::product_dto::{
    CreateProductDto, DeleteProductResponseDto, ProductResponseDto, SearchQueryDto,
    UpdateProductDto,
};
use app_core::error::Error;
use app_core::pagination::{Pagination, PaginationResponseDto};
use axum::extract::{Path, Query, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};

use crate::state::AppState;

/// Registers HTTP routes for product operations and attaches shared application state.
///
/// The returned `Router` exposes endpoints for creating, deleting, updating, retrieving,
/// searching, and listing products, and is already wired to use the provided `AppState`.
///
/// # Examples
///
/// ```no_run
/// use crate::AppState;
/// use crate::handlers::product::routes;
///
/// // Construct your application state (implementation-specific).
/// let state = /* AppState::new(...) */ todo!();
/// let router = routes(state);
/// ```
pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_product))
        .route("/{id}", delete(delete_product))
        .route("/{id}", get(get_product_by_id))
        .route("/search", get(search_products))
        .route("/category/{id}", get(get_product_by_category_id))
        .route("/all", get(get_all_products))
        .route("/update", post(update_product_detail))
        .with_state(state)
}

/// Creates a new product from the provided payload and returns its representation.
///
/// # Returns
///
/// The created product as `ProductResponseDto`.
///
/// # Examples
///
/// ```
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// use axum::extract::{State, Json};
/// // `app_state` and `payload` should be constructed according to your application context.
/// let app_state: AppState = /* ... */;
/// let payload = CreateProductDto { /* ... */ };
/// let response = create_product(State(app_state), Json(payload)).await?;
/// let created_product: ProductResponseDto = response.0;
/// # Ok(()) }
/// ```
pub async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductDto>,
) -> Result<Json<ProductResponseDto>, Error> {
    let model = state.product_service.create_new(payload).await?;
    Ok(Json(model.into()))
}

/// Deletes a product by its ID and returns metadata about the deleted resource.
///
/// # Returns
///
/// `DeleteProductResponseDto` containing the deleted product's `product_id` and `updated_at` timestamp.
///
/// # Examples
///
/// ```no_run
/// use axum::{extract::{State, Path}, Json};
/// // Assuming `state` is available and `product_id` is an i64:
/// // let result = delete_product(State(state), Path(product_id)).await;
/// // if let Ok(Json(dto)) = result {
/// //     println!("deleted id: {}, updated_at: {}", dto.product_id, dto.updated_at);
/// // }
/// ```
pub async fn delete_product(
    State(state): State<AppState>,
    Path(product_id): Path<i64>,
) -> Result<Json<DeleteProductResponseDto>, Error> {
    match state.product_service.delete_by_id(product_id).await {
        Ok(product) => Ok(Json(DeleteProductResponseDto {
            product_id,
            updated_at: product.updated_at,
        })),
        Err(err) => Err(err),
    }
}
/// Updates an existing product's details and returns the updated product as JSON.
///
/// # Examples
///
/// ```no_run
/// # use axum::{extract::State, Json};
/// # use crate::AppState;
/// # use crate::dto::{UpdateProductDto, ProductResponseDto};
/// # async fn example(state: AppState) {
/// let payload = UpdateProductDto { /* fill fields */ };
/// let response = crate::handlers::product::update_product_detail(State(state), Json(payload)).await.unwrap();
/// let _updated: ProductResponseDto = response.0;
/// # }
/// ```
pub async fn update_product_detail(
    State(state): State<AppState>,
    Json(payload): Json<UpdateProductDto>,
) -> Result<Json<ProductResponseDto>, Error> {
    let model = state.product_service.update_detail_by_id(payload).await?;
    Ok(Json(model.into()))
}

/// Retrieve all products using the provided pagination and return them as a paginated JSON response.
///
/// On success, the response contains the list of products converted to `ProductResponseDto`, the total
/// number of pages, and the page size derived from the request.
///
/// # Examples
///
/// ```no_run
/// use axum::Json;
/// use crate::handlers::product::get_all_products;
/// use crate::state::AppState;
/// use crate::dto::{Pagination, PaginationResponseDto, ProductResponseDto};
///
/// # async fn example(state: AppState) {
/// let pagination = Pagination { page: 1, page_size: 10 };
/// // Call the handler like an endpoint (extractors are constructed by Axum in real requests).
/// let _result: Result<Json<PaginationResponseDto<ProductResponseDto>>, _> =
///     get_all_products(axum::extract::State(state), axum::extract::Query(pagination)).await;
/// # }
/// ```
pub async fn get_all_products(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginationResponseDto<ProductResponseDto>>, Error> {
    let (products, num_pages) = state.product_service.get_all(pagination).await?;

    let response = products
        .into_iter()
        .map(|model| model.into())
        .collect::<Vec<_>>();

    Ok(Json(PaginationResponseDto::new(
        response,
        num_pages,
        pagination.page_size,
    )))
}

/// Fetches a product by its ID and returns it as a JSON response.
///
/// Returns a `Json<ProductResponseDto>` containing the requested product on success, or an `Error` if the product cannot be retrieved.
///
/// # Examples
///
/// ```ignore
/// // assuming `state` is an `AppState` and an async runtime is available
/// use axum::extract::{State, Path};
///
/// let result = get_product_by_id(State(state), Path(1)).await;
/// let json = result.unwrap();
/// ```
pub async fn get_product_by_id(
    State(state): State<AppState>,
    Path(product_id): Path<i64>,
) -> Result<Json<ProductResponseDto>, Error> {
    let model = state.product_service.get_by_id(product_id).await?;
    Ok(Json(model.into()))
}

/// Retrieve products for a category with pagination.
///
/// Returns a `PaginationResponseDto` containing `ProductResponseDto` items for the specified
/// `category_id`, the total number of pages available for the query, and the page size requested.
pub async fn get_product_by_category_id(
    State(state): State<AppState>,
    Path(category_id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginationResponseDto<ProductResponseDto>>, Error> {
    let (products, num_pages) = state
        .product_service
        .get_by_category_id(category_id, pagination)
        .await?;

    let response = products
        .into_iter()
        .map(|model| model.into())
        .collect::<Vec<_>>();

    Ok(Json(PaginationResponseDto::new(
        response,
        num_pages,
        pagination.page_size,
    )))
}

/// Searches for products matching the given query and returns a paginated list of product DTOs.
///
/// The handler delegates the search to the product service, converts domain models into
/// `ProductResponseDto`, and wraps the results in a `PaginationResponseDto` containing the
/// items, the total number of pages, and the page size from the request.
///
/// # Examples
///
/// ```ignore
/// // In an async test or handler context:
/// let state = /* AppState with product_service configured */;
/// let query = SearchQueryDto { q: Some("laptop".into()) };
/// let pagination = Pagination { page: 1, page_size: 20 };
/// let result = search_products(State(state), Query(query), Query(pagination)).await;
/// match result {
///     Ok(Json(pagination_resp)) => {
///         assert!(pagination_resp.items.len() <= 20);
///     }
///     Err(e) => panic!("search failed: {:?}", e),
/// }
/// ```
pub async fn search_products(
    State(state): State<AppState>,
    Query(query): Query<SearchQueryDto>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginationResponseDto<ProductResponseDto>>, Error> {
    let (products, num_pages) = state.product_service.search(query, pagination).await?;

    let response = products
        .into_iter()
        .map(|model| model.into())
        .collect::<Vec<_>>();

    Ok(Json(PaginationResponseDto::new(
        response,
        num_pages,
        pagination.page_size,
    )))
}
