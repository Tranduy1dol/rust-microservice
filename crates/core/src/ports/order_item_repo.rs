use async_trait::async_trait;

#[async_trait]
pub trait OrderItemRepository: Send + Sync {}
