use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::validator::validate_password_strength;

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateAdminRequestDto {
    #[validate(email(message = "Email format is invalid"))]
    pub email: String,
    #[validate(
        length(
            min = 8,
            max = 128,
            message = "Password must be between 8 and 128 characters long"
        ),
        custom(
            function = "validate_password_strength",
            message = "Password must contain at least 1 number and 1 uppercase letter"
        )
    )]
    pub password: String,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub confirm_password: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdminResponseDto {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub level: i32,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAdminLevelRequestDto {
    pub email: String,
    pub level: i32,
}
