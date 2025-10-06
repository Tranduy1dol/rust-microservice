use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductRequestDto {
    pub name: String,
    pub category_id: Option<i64>,
    pub price: Decimal,
    pub description: Option<String>,
    pub stock_quantity: i32,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditProductRequestDto {
    pub product_id: i64,
    pub name: Option<String>,
    pub category_id: Option<i64>,
    pub price: Option<Decimal>,
    pub description: Option<String>,
    pub stock_quantity: Option<i32>,
}
