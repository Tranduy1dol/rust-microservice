use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AppEvent {
    UserRegistered {
        user_id: i64,
        email: String,
    },
    OrderPlaced {
        order_id: i64,
        user_id: i64,
        total: rust_decimal::Decimal,
    },
}

impl AppEvent {
    /// Get the event variant name as a static string.
    ///
    /// # Returns
    ///
    /// The variant name as a `'static` string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::events::AppEvent;
    /// let e = AppEvent::UserRegistered { user_id: 1, email: "a@b.com".into() };
    /// assert_eq!(e.name(), "UserRegistered");
    /// let o = AppEvent::OrderPlaced { order_id: 2, user_id: 1, total: rust_decimal::Decimal::new(100, 2) };
    /// assert_eq!(o.name(), "OrderPlaced");
    /// ```
    pub fn name(&self) -> &'static str {
        match self {
            AppEvent::UserRegistered { .. } => "UserRegistered",
            AppEvent::OrderPlaced { .. } => "OrderPlaced",
        }
    }
}
