use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveValue::Set, TransactionTrait, prelude::*};

use app_core::{error::Error, ports::user_repo::UserRepository};
use entities::{cart, prelude::*, user};

#[derive(Clone)]
pub struct SeaOrmUserRepo {
    db: DatabaseConnection,
}

impl SeaOrmUserRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepo {
    async fn create_new(
        &self,
        user_name: String,
        email: String,
        password_hash: String,
        first_name: String,
        last_name: String,
        address: String,
    ) -> Result<user::Model, Error> {
        let txn =
            self.db.begin().await.map_err(|_| {
                Error::internal("Database failed to start transaction.".to_string())
            })?;

        let new_user = user::ActiveModel {
            username: Set(user_name),
            email: Set(email.clone()),
            password_hash: Set(password_hash),
            first_name: Set(first_name),
            last_name: Set(last_name),
            address: Set(address),
            ..Default::default()
        };
        let user_model = new_user
            .insert(&txn)
            .await
            .map_err(|_| Error::internal("Database failed to insert user".to_string()))?;

        let new_cart = cart::ActiveModel {
            user_id: Set(user_model.id),
            ..Default::default()
        };
        new_cart.insert(&txn).await.map_err(|_| {
            Error::internal("Database failed to insert user's new cart".to_string())
        })?;

        txn.commit()
            .await
            .map_err(|_| Error::internal("Database failed to commit transaction".to_string()))?;

        Ok(user_model)
    }

    async fn get_by_email(&self, email: String) -> Result<user::Model, Error> {
        User::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|_| Error::internal("Internal error".to_string()))?
            .ok_or(Error::not_found("User not found".to_string()))
    }

    async fn update_password(
        &self,
        user_id: i64,
        new_password_hash: String,
    ) -> Result<user::Model, Error> {
        let models = User::update_many()
            .filter(user::Column::Id.eq(user_id))
            .col_expr(user::Column::PasswordHash, Expr::value(new_password_hash))
            .col_expr(
                user::Column::UpdatedAt,
                Expr::value(Utc::now().timestamp_millis()),
            )
            .exec_with_returning(&self.db)
            .await?;

        models
            .into_iter()
            .next()
            .ok_or(Error::from(DbErr::Custom("User not found".to_string())))
    }
}
