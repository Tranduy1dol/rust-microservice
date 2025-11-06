use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
    Json, Router,
};

use crate::{
    app::AppState,
    auth::admin::{AdminAuth, AdminScopes, RouteState},
    errors::Error,
    routes::product::dtos::{
        request::{CreateProductRequestDto, EditProductRequestDto, SearchQueryDto},
        response::{DeleteProductRequestDto, ProductResponseDto},
    },
    utils::dtos::PaginationDto,
};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_products))
        .route("/:id", get(get_product_by_id))
        .route("/search", get(search_products))
}

// pub fn create_admin_route() -> Router {
//     Router::new()
//         .route(
//             "/",
//             post(create_product).with_state(RouteState::new(AdminScopes::ProductsWrite)),
//         )
//         .route(
//             "/",
//             delete(delete_product).with_state(RouteState::new(AdminScopes::ProductsWrite)),
//         )
//         .route(
//             "/",
//             patch(edit_product_detail).with_state(RouteState::new(AdminScopes::ProductsWrite)),
//         )
// }

pub async fn create_product(
    State(state): State<AppState>,
    AdminAuth(_claims): AdminAuth,
    Json(request): Json<CreateProductRequestDto>,
) -> Result<Json<ProductResponseDto>, Error> {
    match state
        .product_repo
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
    State(state): State<AppState>,
    AdminAuth(_claims): AdminAuth,
    Path(product_id): Path<i64>,
) -> Result<Json<DeleteProductRequestDto>, Error> {
    match state.product_repo.delete_product_by_id(product_id).await {
        Ok(product) => Ok(Json(DeleteProductRequestDto {
            product_id,
            updated_at: product.updated_at,
        })),
        Err(err) => Err(err),
    }
}
pub async fn edit_product_detail(
    State(state): State<AppState>,
    AdminAuth(_claims): AdminAuth,
    Json(request): Json<EditProductRequestDto>,
) -> Result<Json<ProductResponseDto>, Error> {
    match state
        .product_repo
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
    State(state): State<AppState>,
    Query(pagination): Query<PaginationDto>,
) -> Result<Json<Vec<ProductResponseDto>>, Error> {
    let (products, _num_pages) = state
        .product_repo
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
    State(state): State<AppState>,
    Path(product_id): Path<i64>,
) -> Result<Json<ProductResponseDto>, Error> {
    let model = state.product_repo.get_product_by_id(product_id).await?;
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
    State(state): State<AppState>,
    Query(query): Query<SearchQueryDto>,
    Query(pagination): Query<PaginationDto>,
) -> Result<Json<Vec<ProductResponseDto>>, Error> {
    let (products, _num_pages) = state
        .product_repo
        .search_products(query.q, pagination.page, pagination.page_size)
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
