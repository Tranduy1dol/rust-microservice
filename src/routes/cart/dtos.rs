use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddItemRequestDto {
    pub product_id: i64,
    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    pub quantity: i32,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateItemRequestDto {
    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    pub quantity: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CartItemResponseDto {
    pub product_id: i64,
    pub quantity: i32,
    pub product_name: String,
    pub price_per_unit: Decimal,
    pub total_price: Decimal,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CartResponseDto {
    pub id: i32,
    pub user_id: i64,
    pub items: Vec<CartItemResponseDto>,
    pub total_amount: Decimal,
}
