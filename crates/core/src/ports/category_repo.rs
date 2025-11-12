use async_trait::async_trait;
use entities::category::Model as CategoryModel;

use crate::error::Error;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn create_new(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<CategoryModel, Error>;

    async fn get_by_id(&self, id: i64) -> Result<CategoryModel, Error>;

    async fn update_by_id(
        &self,
        id: i64,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<CategoryModel, Error>;

    async fn delete_by_id(&self, id: i64) -> Result<CategoryModel, Error>;
}
