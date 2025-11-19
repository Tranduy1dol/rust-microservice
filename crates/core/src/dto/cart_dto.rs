use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItemDto {
    pub product_id: i64,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartDto {
    pub user_id: i64,
    pub items: Vec<CartItemDto>,
}

impl CartDto {
    /// Creates a new `CartDto` for the specified user with an empty item list.
    ///
    /// # Examples
    ///
    /// ```
    /// let cart = CartDto::new(42);
    /// assert_eq!(cart.user_id, 42);
    /// assert!(cart.items.is_empty());
    /// ```
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            items: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddCartItemDto {
    #[validate(range(min = 1))]
    pub product_id: i64,
    #[validate(range(min = 1))]
    pub quantity: i32,
}
