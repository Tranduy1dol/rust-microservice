use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};
use validator::Validate;

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductDto {
    pub name: String,
    pub category_id: Option<i64>,
    pub price: Decimal,
    pub description: Option<String>,
    pub stock_quantity: i32,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProductDto {
    pub product_id: i64,
    pub name: Option<String>,
    pub category_id: Option<i64>,
    pub price: Option<Decimal>,
    pub description: Option<String>,
}

fn trim_and_sanitize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.trim().to_string())
}

#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SearchQueryDto {
    #[validate(regex(
        path = r#"regex::Regex::new(r"^[a-zA-Z0-9 ]*$").unwrap()"#,
        message = "Search query can only contain alphanumeric characters and spaces."
    ))]
    #[serde(deserialize_with = "trim_and_sanitize")]
    pub q: String,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
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
pub struct DeleteProductResponseDto {
    pub product_id: i64,
    pub updated_at: i64,
}
