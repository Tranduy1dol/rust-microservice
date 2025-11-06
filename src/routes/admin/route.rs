use axum::{
    extract::State,
    routing::{patch, post},
    Json, Router,
};

use crate::{
    app::AppState,
    auth::jwt::generate_jwt_token,
    config::JWT_CONFIG,
    errors::Error,
    routes::admin::dtos::{
        AdminLoginRequestDto, AdminLoginResponseDto, AdminResponseDto, CreateAdminRequestDto,
        UpdateAdminLevelRequestDto,
    },
};

pub fn create_route() -> Router<AppState> {
    Router::new()
        .route("/", post(create_admin))
        .route("/level", patch(update_admin_level))
}

pub async fn create_admin(
    State(state): State<AppState>,
    Json(request): Json<CreateAdminRequestDto>,
) -> Result<Json<AdminResponseDto>, Error> {
    match state
        .admin_repo
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
    State(state): State<AppState>,
    Json(request): Json<UpdateAdminLevelRequestDto>,
) -> Result<Json<AdminResponseDto>, Error> {
    match state
        .admin_repo
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

pub async fn admin_login(
    State(state): State<AppState>,
    Json(request): Json<AdminLoginRequestDto>,
) -> Result<Json<AdminLoginResponseDto>, Error> {
    let admin = state.admin_repo.get_admin_by_email(request.email).await?;
    match bcrypt::verify(&request.password, admin.password_hash.as_str()) {
        Ok(success) => {
            if success {
                let token = generate_jwt_token(
                    admin.id,
                    admin.email.as_str(),
                    admin.name.as_str(),
                    Some(admin.level),
                    &JWT_CONFIG,
                )
                .map_err(Error::from)?;

                Ok(Json(AdminLoginResponseDto {
                    id: admin.id.to_string(),
                    name: admin.name,
                    token,
                }))
            } else {
                Err(Error::unauthorized("Invalid password".to_string()))
            }
        }
        Err(err) => Err(Error::from(err)),
    }
}
