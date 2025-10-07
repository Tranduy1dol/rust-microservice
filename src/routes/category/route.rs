use axum::{
    extract::Path,
    routing::{delete, patch, post},
    Json, Router,
};
use chrono::Utc;

use crate::{
    auth::admin::{AdminAuth, AdminScopes, RouteState},
    errors::Error,
    repositories::category_repository::CATEGORY_REPOSITORY,
    routes::category::dtos::{
        request::{CreateCategoryRequestDto, EditCreateCategoryRequestDto},
        response::{CategoryResponseDto, DeleteCategoryResponseDto},
    },
};

pub fn create_route() -> Router {
    Router::new()
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
