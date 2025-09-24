use axum::{
    routing::{get, post},
    Json, Router,
};

use crate::{
    auth::jwt::{generate_jwt_token, Auth},
    config::JWT_CONFIG,
    errors::Error,
    repositories::user_repository::USER_REPOSITORY,
    routes::user::dtos::{
        request::{LoginRequestDto, RegisterRequestDto, ResetPasswordRequestDto},
        response::{
            GetUserProfileResponseDto, LoginResponseDto, RegisterResponseDto,
            ResetPasswordResponseDto,
        },
    },
};
pub fn create_route() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/profile", get(profile))
        .route("/reset-password", post(reset_password))
}

pub async fn register(
    Json(request): Json<RegisterRequestDto>,
) -> Result<Json<RegisterResponseDto>, Error> {
    match USER_REPOSITORY
        .create_new_user(
            request.user_name,
            request.email,
            request.password,
            request.first_name,
            request.last_name,
            request.address,
        )
        .await
    {
        Ok(user) => Ok(Json(RegisterResponseDto {
            user_id: user.id,
            email: user.email,
        })),
        Err(err) => Err(err),
    }
}

pub async fn login(Json(request): Json<LoginRequestDto>) -> Result<Json<LoginResponseDto>, Error> {
    let user = USER_REPOSITORY.get_user_by_email(request.email).await?;
    match bcrypt::verify(&request.password, user.password_hash.as_str()) {
        Ok(result) => {
            if result {
                let token =
                    generate_jwt_token(user.id, &user.email, &user.username, None, &JWT_CONFIG)
                        .unwrap();

                Ok(Json(LoginResponseDto {
                    user_id: user.id,
                    token,
                }))
            } else {
                Err(Error::unauthorized("Invalid password!".to_string()))
            }
        }
        Err(err) => Err(Error::from(err)),
    }
}

pub async fn profile(Auth(claim): Auth) -> Result<Json<GetUserProfileResponseDto>, Error> {
    match USER_REPOSITORY.get_user_by_email(claim.email).await {
        Ok(user) => Ok(Json(GetUserProfileResponseDto {
            user_id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            address: user.address,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })),
        Err(err) => Err(err),
    }
}

pub async fn reset_password(
    Auth(claim): Auth,
    Json(request): Json<ResetPasswordRequestDto>,
) -> Result<Json<ResetPasswordResponseDto>, Error> {
    let user = USER_REPOSITORY
        .get_user_by_email(claim.email.clone())
        .await?;
    match bcrypt::verify(&request.current_password, user.password_hash.as_str()) {
        Ok(result) => {
            if result {
                USER_REPOSITORY
                    .update_user_password(claim.email, request.new_password)
                    .await?;

                Ok(Json(ResetPasswordResponseDto { user_id: user.id }))
            } else {
                Err(Error::unauthorized("Invalid password!".to_string()))
            }
        }
        Err(err) => Err(Error::from(err)),
    }
}
