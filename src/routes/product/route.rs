use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Json, Router,
};

use crate::{
    auth::admin::{AdminAuth, AdminScopes, RouteState},
    errors::Error,
    repositories::product_repository::PRODUCT_REPOSITORY,
    routes::product::dtos::{
        request::{CreateProductRequestDto, EditProductRequestDto, SearchQueryDto},
        response::{DeleteProductRequestDto, ProductResponseDto},
    },
    utils::dtos::PaginationDto,
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(get_all_products))
        .route("/:id", get(get_product_by_id))
        .route("/search", get(search_products))
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

pub async fn get_all_products(
    Query(pagination): Query<PaginationDto>,
) -> Result<Json<Vec<ProductResponseDto>>, Error> {
    let (products, _num_pages) = PRODUCT_REPOSITORY
        .get_all_products(pagination.page, pagination.page_size)
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

pub async fn get_product_by_id(
    Path(product_id): Path<i64>,
) -> Result<Json<ProductResponseDto>, Error> {
    let model = PRODUCT_REPOSITORY.get_product_by_id(product_id).await?;
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

pub async fn search_products(
    Query(query): Query<SearchQueryDto>,
    Query(pagination): Query<PaginationDto>,
) -> Result<Json<Vec<ProductResponseDto>>, Error> {
    let (products, _num_pages) = PRODUCT_REPOSITORY
        .search_products_by_name(query.q, pagination.page, pagination.page_size)
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
