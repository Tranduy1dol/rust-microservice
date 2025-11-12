use async_trait::async_trait;

#[async_trait]
pub trait OrderRepository: Send + Sync {}
