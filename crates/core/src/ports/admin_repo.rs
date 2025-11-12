use async_trait::async_trait;
use entities::admin::Model as AdminModel;

use crate::error::Error;

#[async_trait]
pub trait AdminRepository: Send + Sync {
    async fn create_new(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<AdminModel, Error>;

    async fn get_admin_by_email(&self, email: String) -> Result<AdminModel, Error>;

    async fn update_level(&self, email: String, level: i32) -> Result<AdminModel, Error>;
}
