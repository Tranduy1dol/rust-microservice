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

pub async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductDto>,
) -> Result<Json<ProductResponseDto>, Error> {
    let model = state.product_service.create_new(payload).await?;
    Ok(Json(model.into()))
}

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
pub async fn update_product_detail(
    State(state): State<AppState>,
    Json(payload): Json<UpdateProductDto>,
) -> Result<Json<ProductResponseDto>, Error> {
    let model = state.product_service.update_detail_by_id(payload).await?;
    Ok(Json(model.into()))
}

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

pub async fn get_product_by_id(
    State(state): State<AppState>,
    Path(product_id): Path<i64>,
) -> Result<Json<ProductResponseDto>, Error> {
    let model = state.product_service.get_by_id(product_id).await?;
    Ok(Json(model.into()))
}

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
