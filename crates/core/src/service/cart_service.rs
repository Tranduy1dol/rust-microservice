use std::sync::Arc;

use crate::dto::cart_dto::{CartDto, CartItemDto};
use crate::error::Error;
use crate::ports::cart_repo::CartRepository;

pub struct CartService {
    cart_repo: Arc<dyn CartRepository>,
}

impl CartService {
    /// Creates a CartService that uses the given cart repository.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// // `repo` must implement `CartRepository`.
    /// let repo = Arc::new(/* your CartRepository implementation */);
    /// let svc = CartService::new(repo);
    /// ```
    pub fn new(cart_repo: Arc<dyn CartRepository>) -> Self {
        Self { cart_repo }
    }

    /// Fetches the cart for the specified user, returning an empty cart if none exists.
    ///
    /// # Returns
    ///
    /// `CartDto` containing the user's cart, or a new empty `CartDto` for the user if no cart was found.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // async context required
    /// let service: CartService = /* obtain CartService instance */;
    /// let cart = service.get_cart(42).await.unwrap();
    /// assert_eq!(cart.user_id, 42);
    /// ```
    pub async fn get_cart(&self, user_id: i64) -> Result<CartDto, Error> {
        let cart = self.cart_repo.get_by_user_id(user_id).await?;
        Ok(cart.unwrap_or_else(|| CartDto::new(user_id)))
    }

    /// Adds the specified quantity of a product to the user's cart, creating the cart or item if necessary and persisting the change.
    ///
    /// The provided `quantity` must be greater than zero. If the cart already contains an item with `product_id`, that item's quantity is increased by `quantity`; otherwise a new item is appended. The updated cart is saved in the repository before being returned.
    ///
    /// # Parameters
    ///
    /// - `quantity`: The number of units to add; must be greater than zero.
    ///
    /// # Returns
    ///
    /// `CartDto` containing the saved cart with the updated items.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::sync::Arc;
    /// # async fn example(cart_service: &crate::service::CartService) {
    /// let updated = cart_service.add_item(1, 42, 3).await.unwrap();
    /// assert!(updated.items.iter().any(|i| i.product_id == 42 && i.quantity >= 3));
    /// # }
    /// ```
    pub async fn add_item(
        &self,
        user_id: i64,
        product_id: i64,
        quantity: i32,
    ) -> Result<CartDto, Error> {
        if quantity <= 0 {
            return Err(Error::bad_request("Quantity must be positive".to_string()));
        }

        let mut cart = self.get_cart(user_id).await?;

        if let Some(item) = cart.items.iter_mut().find(|i| i.product_id == product_id) {
            item.quantity += quantity;
        } else {
            cart.items.push(CartItemDto {
                product_id,
                quantity,
            });
        }

        self.cart_repo.save(cart.clone()).await?;
        Ok(cart)
    }

    /// Removes all items with the given product ID from the user's cart and persists the updated cart.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example(svc: &crate::service::CartService) -> Result<(), crate::error::Error> {
    /// let user_id = 1;
    /// let product_id = 42;
    /// let updated = svc.remove_item(user_id, product_id).await?;
    /// // `updated` is the cart after the item(s) with `product_id` have been removed
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// `CartDto` with any items matching `product_id` removed; if no matching items existed, returns the cart unchanged.
    pub async fn remove_item(&self, user_id: i64, product_id: i64) -> Result<CartDto, Error> {
        let mut cart = self.get_cart(user_id).await?;

        cart.items.retain(|i| i.product_id != product_id);

        self.cart_repo.save(cart.clone()).await?;
        Ok(cart)
    }

    /// Deletes the cart for the given user from the repository.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example(service: &crate::service::CartService) -> Result<(), crate::Error> {
    /// service.clear_cart(42).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, `Err(Error)` if the repository operation fails.
    pub async fn clear_cart(&self, user_id: i64) -> Result<(), Error> {
        self.cart_repo.delete_by_user_id(user_id).await
    }
}
