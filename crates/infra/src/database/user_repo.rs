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
    /// Constructs a new SeaOrmUserRepo that wraps the given database connection.
    ///
    /// # Examples
    ///
    /// ```
    /// use infra::database::user_repo::SeaOrmUserRepo;
    /// let db: sea_orm::DatabaseConnection = /* obtain or mock a DatabaseConnection */ unimplemented!();
    /// let repo = SeaOrmUserRepo::new(db);
    /// ```
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepo {
    /// Create a new user and an associated empty cart within a single database transaction.
    ///
    /// On success returns the created user model.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // `repo` is a SeaOrmUserRepo instance connected to a DatabaseConnection.
    /// let created = repo.create_new(
    ///     "alice".to_string(),
    ///     "alice@example.com".to_string(),
    ///     "hashed_password".to_string(),
    ///     "Alice".to_string(),
    ///     "Smith".to_string(),
    ///     "123 Main St".to_string(),
    /// ).await.unwrap();
    /// assert_eq!(created.email, "alice@example.com");
    /// ```
    async fn create_new(
        &self,
        user_name: String,
        email: String,
        password_hash: String,
        first_name: String,
        last_name: String,
        address: String,
    ) -> Result<user::Model, Error> {
        let txn = self.db.begin().await.map_err(|e| {
            tracing::error!("Failed to start transaction: {}", e);
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
        let user_model = new_user.insert(&txn).await.map_err(|e| {
            tracing::error!("Failed to insert user: {}", e);
            Error::internal("Database failed to insert user".to_string())
        })?;

        let new_cart = cart::ActiveModel {
            user_id: Set(user_model.id),
            ..Default::default()
        };
        new_cart.insert(&txn).await.map_err(|e| {
            tracing::error!("Failed to insert cart: {}", e);
            Error::internal("Database failed to insert user's new cart".to_string())
        })?;

        txn.commit().await.map_err(|e| {
            tracing::error!("Database failed to commit transaction: {}", e);
            Error::internal(format!("Database failed to commit transaction: {}", e))
        })?;

        Ok(user_model)
    }

    /// Finds a user by email.
    ///
    /// Returns the matching `user::Model` when a user with the specified email exists;
    /// returns `Err(Error::not_found)` if no such user exists, or `Err(Error::internal)` if the database query fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // inside an async context
    /// let repo: SeaOrmUserRepo = /* created elsewhere */;
    /// let result = repo.get_by_email("alice@example.com".to_string()).await;
    /// match result {
    ///     Ok(user) => println!("Found user: {}", user.username),
    ///     Err(e) => eprintln!("Error: {:?}", e),
    /// }
    /// ```
    async fn get_by_email(&self, email: String) -> Result<user::Model, Error> {
        User::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e| {
                tracing::error!("Database query failed: {}", e);
                Error::internal("Internal error".to_string())
            })?
            .ok_or(Error::not_found("User not found".to_string()))
    }

    /// Updates the stored password hash and `UpdatedAt` timestamp for a user and returns the updated user record.
    ///
    /// On success returns the updated `user::Model`. Returns an `Error` if the user does not exist or if a database error occurs.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # async fn example() {
    /// let repo = /* SeaOrmUserRepo::new(db) */;
    /// let updated = repo.update_password(42, "new_hash".to_string()).await.unwrap();
    /// assert_eq!(updated.id, 42);
    /// # }
    /// ```
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
