use axum::{
    routing::{patch, post},
    Json, Router,
};

use crate::{
    errors::Error,
    repositories::admin_repository::ADMIN_REPOSITORY,
    routes::admin::dtos::{AdminResponseDto, CreateAdminRequestDto, UpdateAdminLevelRequestDto},
};

pub fn create_route() -> Router {
    Router::new()
        .route("/", post(create_admin))
        .route("/level", patch(update_admin_level))
}

pub async fn create_admin(
    Json(request): Json<CreateAdminRequestDto>,
) -> Result<Json<AdminResponseDto>, Error> {
    match ADMIN_REPOSITORY
        .create_new_admin(request.name, request.email, request.password)
        .await
    {
        Ok(admin) => Ok(Json(AdminResponseDto {
            id: admin.id,
            name: admin.name,
            email: admin.email,
            level: admin.level,
        })),
        Err(err) => Err(err),
    }
}

pub async fn update_admin_level(
    Json(request): Json<UpdateAdminLevelRequestDto>,
) -> Result<Json<AdminResponseDto>, Error> {
    match ADMIN_REPOSITORY
        .update_admin_level(request.email, request.level)
        .await
    {
        Ok(admin) => Ok(Json(AdminResponseDto {
            id: admin.id,
            name: admin.name,
            email: admin.email,
            level: admin.level,
        })),
        Err(err) => Err(err),
    }
}
