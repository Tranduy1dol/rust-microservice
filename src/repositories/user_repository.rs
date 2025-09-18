use std::sync::LazyLock;

use chrono::Utc;
use entities::user::{ActiveModel as UserActiveModel, Entity as User, Model as UserModel};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, NotSet, Set};

use crate::database::get_database;

pub struct UserRepository {
    database: DatabaseConnection,
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl UserRepository {
    pub fn new() -> Self {
        Self {
            database: get_database().to_owned(),
        }
    }

    pub async fn create_new_user(
        &self,
        user_name: String,
        email: String,
        password_hash: String,
        first_name: String,
        last_name: String,
        address: String,
    ) -> Result<UserModel, DbErr> {
        let now = Utc::now().timestamp_millis();
        let active_model = UserActiveModel {
            id: NotSet,
            username: Set(user_name),
            email: Set(email),
            password_hash: Set(password_hash),
            first_name: Set(first_name),
            last_name: Set(last_name),
            address: Set(address),
            created_at: Set(now),
            updated_at: Set(now),
        };

        User::insert(active_model)
            .exec_with_returning(&self.database)
            .await
    }
}

pub static USER_REPOSITORY: LazyLock<UserRepository> = LazyLock::new(UserRepository::new);
