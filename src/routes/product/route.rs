use axum::{
    extract::Path,
    routing::{delete, patch, post},
    Json, Router,
};

use crate::{
    auth::admin::{AdminAuth, AdminScopes, RouteState},
    errors::Error,
    repositories::product_repository::PRODUCT_REPOSITORY,
    routes::product::dtos::{
        request::{CreateProductRequestDto, EditProductRequestDto},
        response::{DeleteProductRequestDto, ProductResponseDto},
    },
};

pub fn create_routes() -> Router {
    Router::new()
        .route(
            "/",
            post(create_product).with_state(RouteState::new(AdminScopes::ProductsWrite)),
        )
        .route(
            "/",
            delete(delete_product).with_state(RouteState::new(AdminScopes::ProductsWrite)),
        )
        .route(
            "/",
            patch(edit_product_detail).with_state(RouteState::new(AdminScopes::ProductsWrite)),
        )
}

pub async fn create_product(
    AdminAuth(_claims): AdminAuth,
    Json(request): Json<CreateProductRequestDto>,
) -> Result<Json<ProductResponseDto>, Error> {
    match PRODUCT_REPOSITORY
        .create_new_product(
            request.name,
            request.description,
            request.price,
            request.category_id,
            request.stock_quantity,
        )
        .await
    {
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
    AdminAuth(_claims): AdminAuth,
    Path(product_id): Path<i64>,
) -> Result<Json<DeleteProductRequestDto>, Error> {
    match PRODUCT_REPOSITORY.delete_product_by_id(product_id).await {
        Ok(product) => Ok(Json(DeleteProductRequestDto {
            product_id,
            updated_at: product.updated_at,
        })),
        Err(err) => Err(err),
    }
}
pub async fn edit_product_detail(
    AdminAuth(_claims): AdminAuth,
    Json(request): Json<EditProductRequestDto>,
) -> Result<Json<ProductResponseDto>, Error> {
    match PRODUCT_REPOSITORY
        .update_product_detail_by_id(
            request.product_id,
            request.name,
            request.description,
            request.price,
            request.category_id,
            request.stock_quantity,
        )
        .await
    {
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
