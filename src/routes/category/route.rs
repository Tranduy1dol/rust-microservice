use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Json, Router,
};
use chrono::Utc;

use crate::{
    auth::admin::{AdminAuth, AdminScopes, RouteState},
    errors::Error,
    repositories::{
        category_repository::CATEGORY_REPOSITORY, product_repository::PRODUCT_REPOSITORY,
    },
    routes::{
        category::dtos::{
            request::{CreateCategoryRequestDto, EditCreateCategoryRequestDto},
            response::{CategoryResponseDto, DeleteCategoryResponseDto},
        },
        product::dtos::response::ProductResponseDto,
    },
    utils::dtos::PaginationDto,
};

pub fn create_route() -> Router {
    Router::new()
        // Route công khai (MỚI)
        .route("/:id/products", get(get_products_by_category))
        .route(
            "/",
            post(create_category).with_state(RouteState::new(AdminScopes::CategoriesWrite)),
        )
        .route(
            "/",
            delete(delete_category).with_state(RouteState::new(AdminScopes::CategoriesWrite)),
        )
        .route(
            "/",
            patch(edit_category).with_state(RouteState::new(AdminScopes::CategoriesWrite)),
        )
}

pub async fn create_category(
    AdminAuth(_claims): AdminAuth,
    Json(request): Json<CreateCategoryRequestDto>,
) -> Result<Json<CategoryResponseDto>, Error> {
    match CATEGORY_REPOSITORY
        .create_new_category(request.name, request.description)
        .await
    {
        Ok(model) => Ok(Json(CategoryResponseDto {
            id: model.id,
            name: model.name,
            description: None,
        })),
        Err(err) => Err(err),
    }
}

pub async fn delete_category(
    AdminAuth(_claims): AdminAuth,
    Path(category_id): Path<i64>,
) -> Result<Json<DeleteCategoryResponseDto>, Error> {
    match CATEGORY_REPOSITORY.delete_category(category_id).await {
        Ok(model) => Ok(Json(DeleteCategoryResponseDto {
            id: model.id,
            deleted_at: Utc::now().timestamp(),
        })),
        Err(err) => Err(err),
    }
}

pub async fn edit_category(
    AdminAuth(_claims): AdminAuth,
    Json(request): Json<EditCreateCategoryRequestDto>,
) -> Result<Json<CategoryResponseDto>, Error> {
    match CATEGORY_REPOSITORY
        .update_category(request.id, request.name, request.description)
        .await
    {
        Ok(model) => Ok(Json(CategoryResponseDto {
            id: model.id,
            name: model.name,
            description: None,
        })),
        Err(err) => Err(err),
    }
}

pub async fn get_products_by_category(
    Path(category_id): Path<i64>,
    Query(pagination): Query<PaginationDto>,
) -> Result<Json<Vec<ProductResponseDto>>, Error> {
    // Gọi hàm từ product_repository
    let (products, _num_pages) = PRODUCT_REPOSITORY
        .get_products_by_category(category_id, pagination.page, pagination.page_size)
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
        .collect();

    Ok(Json(response))
}
