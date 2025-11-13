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
    match state.product_service.create_new(payload).await {
        Ok(model) => Ok(Json(ProductResponseDto {
            id: model.id,
            name: model.name,
            description: model.description,
            price: model.price,
            stock_quantity: model.stock_quantity,
            category_id: model.category_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })),
        Err(err) => Err(err),
    }
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
    match state.product_service.update_detail_by_id(payload).await {
        Ok(product) => Ok(Json(ProductResponseDto {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            stock_quantity: product.stock_quantity,
            category_id: product.category_id,
            created_at: product.created_at,
            updated_at: product.updated_at,
        })),
        Err(err) => Err(err),
    }
}

pub async fn get_all_products(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginationResponseDto<ProductResponseDto>>, Error> {
    let (products, num_pages) = state.product_service.get_all(pagination).await?;

    let response = products
        .into_iter()
        .map(|model| ProductResponseDto {
            id: model.id,
            name: model.name,
            description: model.description,
            price: model.price,
            stock_quantity: model.stock_quantity,
            category_id: model.category_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
        .collect::<Vec<_>>();

    Ok(Json(PaginationResponseDto::new(response, num_pages)))
}

pub async fn get_product_by_id(
    State(state): State<AppState>,
    Path(product_id): Path<i64>,
) -> Result<Json<ProductResponseDto>, Error> {
    let model = state.product_service.get_by_id(product_id).await?;
    Ok(Json(ProductResponseDto {
        id: model.id,
        name: model.name,
        description: model.description,
        price: model.price,
        stock_quantity: model.stock_quantity,
        category_id: model.category_id,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }))
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
        .map(|model| ProductResponseDto {
            id: model.id,
            name: model.name,
            description: model.description,
            price: model.price,
            stock_quantity: model.stock_quantity,
            category_id: model.category_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
        .collect::<Vec<_>>();

    Ok(Json(PaginationResponseDto::new(response, num_pages)))
}

pub async fn search_products(
    State(state): State<AppState>,
    Query(query): Query<SearchQueryDto>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginationResponseDto<ProductResponseDto>>, Error> {
    let (products, num_pages) = state.product_service.search(query, pagination).await?;

    let response = products
        .into_iter()
        .map(|model| ProductResponseDto {
            id: model.id,
            name: model.name,
            description: model.description,
            price: model.price,
            stock_quantity: model.stock_quantity,
            category_id: model.category_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
        .collect::<Vec<_>>();

    Ok(Json(PaginationResponseDto::new(response, num_pages)))
}
