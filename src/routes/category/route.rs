use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use chrono::Utc;

use crate::{
    app::AppState,
    auth::admin::{AdminAuth, AdminScopes, RouteState},
    errors::Error,
    routes::{
        category::dtos::{
            request::{CreateCategoryRequestDto, EditCreateCategoryRequestDto},
            response::{CategoryResponseDto, DeleteCategoryResponseDto},
        },
        product::dtos::response::ProductResponseDto,
    },
    utils::dtos::PaginationDto,
};

pub fn create_route() -> Router<AppState> {
    Router::new().route("/:id/products", get(get_products_by_category))
}

// pub fn create_admin_route() -> Router {
//     Router::new()
//         .route(
//             "/",
//             post(create_category).with_state(RouteState::new(AdminScopes::CategoriesWrite)),
//         )
//         .route(
//             "/",
//             delete(delete_category).with_state(RouteState::new(AdminScopes::CategoriesWrite)),
//         )
//         .route(
//             "/",
//             patch(edit_category).with_state(RouteState::new(AdminScopes::CategoriesWrite)),
//         )
// }

pub async fn create_category(
    State(state): State<AppState>,
    AdminAuth(_claims): AdminAuth,
    Json(request): Json<CreateCategoryRequestDto>,
) -> Result<Json<CategoryResponseDto>, Error> {
    match state
        .category_repo
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
    State(state): State<AppState>,
    AdminAuth(_claims): AdminAuth,
    Path(category_id): Path<i64>,
) -> Result<Json<DeleteCategoryResponseDto>, Error> {
    match state.category_repo.delete_category(category_id).await {
        Ok(model) => Ok(Json(DeleteCategoryResponseDto {
            id: model.id,
            deleted_at: Utc::now().timestamp(),
        })),
        Err(err) => Err(err),
    }
}

pub async fn edit_category(
    State(state): State<AppState>,
    AdminAuth(_claims): AdminAuth,
    Json(request): Json<EditCreateCategoryRequestDto>,
) -> Result<Json<CategoryResponseDto>, Error> {
    match state
        .category_repo
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
    State(state): State<AppState>,
    Path(category_id): Path<i64>,
    Query(pagination): Query<PaginationDto>,
) -> Result<Json<Vec<ProductResponseDto>>, Error> {
    let (products, _num_pages) = state
        .product_repo
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
