use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::validator::validate_password_strength;

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequestDto {
    pub user_name: String,
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
    pub first_name: String,
    pub last_name: String,
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequestDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordRequestDto {
    pub current_password: String,
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
    pub new_password: String,
    #[validate(must_match(other = "new_password", message = "Passwords do not match"))]
    pub confirm_password: String,
}
