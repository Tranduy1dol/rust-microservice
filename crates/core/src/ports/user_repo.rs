use async_trait::async_trait;
use entities::user::Model as UserModel;

use crate::error::Error;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_new_user(
        &self,
        user_name: String,
        email: String,
        password_hash: String,
        first_name: String,
        last_name: String,
        address: String,
    ) -> Result<UserModel, Error>;

    async fn get_user_by_email(&self, email: String) -> Result<UserModel, Error>;

    async fn update_user_password(
        &self,
        user_id: i64,
        new_password_hash: String,
    ) -> Result<UserModel, Error>;
}
