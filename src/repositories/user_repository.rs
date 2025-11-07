use chrono::Utc;
use entities::{
    cart::{ActiveModel as CartActiveModel, Entity as Cart},
    user::{ActiveModel as UserActiveModel, Column, Entity as User, Model as UserModel, Model},
};
use sea_orm::{
    sea_query::Expr, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, NotSet, QueryFilter, Set,
    TransactionTrait,
};

use crate::errors::Error;

pub struct UserRepository {
    database: DatabaseConnection,
}

impl UserRepository {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }

    pub async fn create_new_user(
        &self,
        user_name: String,
        email: String,
        password: String,
        first_name: String,
        last_name: String,
        address: String,
    ) -> Result<UserModel, Error> {
        if User::find()
            .filter(Column::Email.eq(email.clone()))
            .one(&self.database)
            .await?
            .is_some()
        {
            return Err(Error::from(DbErr::Custom(
                "User already exists!".to_string(),
            )));
        }

        let password_hash = bcrypt::hash(password.as_bytes(), bcrypt::DEFAULT_COST)?;

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

        let txn = self.database.begin().await?;

        let user_model = User::insert(active_model).exec_with_returning(&txn).await?;

        let now = Utc::now().timestamp_millis();
        let cart_active_model = CartActiveModel {
            id: NotSet,
            user_id: Set(user_model.id),
            created_at: Set(now),
            updated_at: Set(now),
        };
        Cart::insert(cart_active_model).exec(&txn).await?;

        txn.commit().await?;

        Ok(user_model)
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<UserModel, Error> {
        let user = User::find()
            .filter(Column::Email.eq(email))
            .one(&self.database)
            .await?;

        if user.is_none() {
            return Err(Error::from(DbErr::Custom("User not found!".to_string())));
        }

        Ok(user.unwrap())
    }

    pub async fn update_user_password(
        &self,
        email: String,
        password: String,
    ) -> Result<Model, Error> {
        let password_hash = bcrypt::hash(password.as_bytes(), bcrypt::DEFAULT_COST)?;
        let models = User::update_many()
            .filter(Column::Email.eq(email))
            .col_expr(Column::PasswordHash, Expr::value(password_hash))
            .col_expr(
                Column::UpdatedAt,
                Expr::value(Utc::now().timestamp_millis()),
            )
            .exec_with_returning(&self.database)
            .await?;

        models
            .into_iter()
            .next()
            .ok_or(Error::from(DbErr::Custom("User not found!".to_string())))
    }
}
