use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::validator::validate_password_strength;

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserDto {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
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
    #[validate(length(
        min = 1,
        max = 100,
        message = "First name must be between 1 and 100 characters"
    ))]
    pub first_name: String,
    #[validate(length(
        min = 1,
        max = 100,
        message = "Last name must be between 1 and 100 characters"
    ))]
    pub last_name: String,
    #[validate(length(max = 500, message = "Address must not exceed 500 characters"))]
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}
