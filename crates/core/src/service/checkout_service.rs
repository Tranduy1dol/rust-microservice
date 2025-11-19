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
    /// Constructs a CheckoutService using the provided shared CartService and CheckoutRepository.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// // assume CartService and CheckoutRepository impl are in scope
    /// let cart_service = Arc::new(CartService::new());
    /// let checkout_repo: Arc<dyn CheckoutRepository> = Arc::new(InMemoryCheckoutRepo::new());
    /// let svc = CheckoutService::new(cart_service, checkout_repo);
    /// ```
    pub fn new(cart_service: Arc<CartService>, checkout_repo: Arc<dyn CheckoutRepository>) -> Self {
        Self {
            cart_service,
            checkout_repo,
        }
    }

    /// Performs checkout for the specified user: creates an order from the user's cart and clears the cart.
    ///
    /// Attempts to retrieve the user's cart, returns an error with message `"Cart is empty"` if the cart contains no items, creates an order from the cart items on success, clears the cart, and returns the created order.
    ///
    /// # Examples
    ///
    /// ```
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

        // Clear cart after successful checkout
        self.cart_service.clear_cart(user_id).await?;

        Ok(order)
    }
}
