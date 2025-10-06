use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductResponseDto {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock_quantity: i32,
    pub category_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProductRequestDto {
    pub product_id: i64,
    pub updated_at: i64,
}
