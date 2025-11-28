use std::sync::Arc;

use entities::order;

use crate::error::Error;
use crate::ports::checkout_repo::CheckoutRepository;
use crate::service::cart_service::CartService;

pub struct CheckoutService {
    cart_service: Arc<CartService>,
    checkout_repo: Arc<dyn CheckoutRepository>,
}

impl CheckoutService {
    /// Create a CheckoutService that uses the provided shared CartService and CheckoutRepository.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::sync::Arc;
    /// // assume CartService and CheckoutRepository impl are in scope
    /// let cart_service = Arc::new(CartService::new());
    /// let checkout_repo: Arc<dyn CheckoutRepository> = Arc::new(InMemoryCheckoutRepo::new());
    /// let svc = CheckoutService::new(cart_service, checkout_repo);
    /// ```ignore
    pub fn new(cart_service: Arc<CartService>, checkout_repo: Arc<dyn CheckoutRepository>) -> Self {
        Self {
            cart_service,
            checkout_repo,
        }
    }

    /// Creates an order from the specified user's cart and attempts to clear the cart.
    ///
    /// Retrieves the user's cart, returns an error with message `"Cart is empty"` if the cart has no items, creates an order from the cart items, and then best-effort clears the cart. Errors from fetching the cart or creating the order are propagated; failures to clear the cart are logged and do not change the returned result.
    ///
    /// # Returns
    ///
    /// The created `order::Model` on success.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // assuming `svc` is a configured `CheckoutService`
    /// let result = futures::executor::block_on(async { svc.checkout(1).await });
    /// assert!(result.is_ok());
    /// ```
    pub async fn checkout(&self, user_id: i64) -> Result<order::Model, Error> {
        let cart = self.cart_service.get_cart(user_id).await?;

        if cart.items.is_empty() {
            return Err(Error::bad_request("Cart is empty".to_string()));
        }

        let order = self.checkout_repo.create_order(user_id, cart.items).await?;

        // Clear cart after successful checkout (best-effort)
        // Don't propagate errors to prevent duplicate orders on retry
        if let Err(e) = self.cart_service.clear_cart(user_id).await {
            tracing::warn!(
                "Failed to clear cart for user {} after creating order {}: {:?}",
                user_id,
                order.id,
                e
            );
        }

        Ok(order)
    }
}