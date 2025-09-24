use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponseDto {
    pub user_id: i64,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseDto {
    pub user_id: i64,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserProfileResponseDto {
    pub user_id: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordResponseDto {
    pub user_id: i64,
}
