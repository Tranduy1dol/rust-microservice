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
