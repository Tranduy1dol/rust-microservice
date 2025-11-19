use lazy_static::lazy_static;
use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};
use validator::Validate;

use crate::validator::validate_price_positive;

#[derive(Serialize, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductDto {
    #[validate(length(min = 1, message = "Product name cannot be empty."))]
    pub name: String,
    pub category_id: Option<i64>,
    #[validate(custom(
        function = "validate_price_positive",
        message = "Price must be positive."
    ))]
    pub price: Decimal,
    pub description: Option<String>,
    #[validate(range(min = 0, message = "Stock quantity cannot be negative."))]
    pub stock_quantity: i32,
}

#[derive(Serialize, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProductDto {
    pub product_id: i64,
    #[validate(length(min = 1, message = "Product name cannot be empty"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub category_id: Option<i64>,
    #[validate(custom(
        function = "validate_price_positive",
        message = "Price must be positive."
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    pub description: Option<String>,
}

/// Trims leading and trailing whitespace from a deserialized string.
///
/// This helper can be used with serde's `deserialize_with` to ensure string fields
/// are trimmed during deserialization. Deserialization errors are forwarded unchanged.
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug, PartialEq)]
/// struct S {
///     #[serde(deserialize_with = "crate::dto::product_dto::trim_and_sanitize")]
///     q: String,
/// }
///
/// let json = r#"{"q":"  hello world  "}"#;
/// let s: S = serde_json::from_str(json).unwrap();
/// assert_eq!(s.q, "hello world");
/// ```
fn trim_and_sanitize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.trim().to_string())
}

lazy_static! {
    static ref SEARCH_QUERY_RE: Regex = Regex::new(r"^[a-zA-Z0-9 ]+$").unwrap();
}

#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SearchQueryDto {
    #[validate(regex(path = *SEARCH_QUERY_RE, message = "Search query can only contain alphanumeric characters and spaces."))]
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

impl From<entities::product::Model> for ProductResponseDto {
    /// Converts an `entities::product::Model` into a `ProductResponseDto`.
    ///
    /// The DTO fields are populated directly from the model's corresponding fields.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `model` is an `entities::product::Model` instance:
    /// // let model = entities::product::Model { /* fields */ };
    /// // let dto = ProductResponseDto::from(model);
    /// // assert_eq!(dto.id, /* expected id */);
    /// ```
    fn from(model: entities::product::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            price: model.price,
            stock_quantity: model.stock_quantity,
            category_id: model.category_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProductResponseDto {
    pub product_id: i64,
    pub updated_at: i64,
}
