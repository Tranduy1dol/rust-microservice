use std::sync::Arc;

use chrono::{Duration, Utc};
use entities::user;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    dto::user_dto::{LoginDto, RegisterUserDto},
    error::Error,
    ports::user_repo::UserRepository,
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i64,
    exp: usize,
    iat: usize,
}

pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
    jwt_secret: String,
}

impl UserService {
    /// Creates a new UserService that uses the provided user repository and JWT secret.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// # use crates_core::service::UserService;
    /// # use crates_core::repository::InMemoryUserRepo;
    /// // `repo` must implement `UserRepository`.
    /// let repo = Arc::new(InMemoryUserRepo::default());
    /// let jwt_secret = "your-jwt-secret".to_string();
    /// let svc = UserService::new(repo, jwt_secret);
    /// ```
    pub fn new(user_repo: Arc<dyn UserRepository>, jwt_secret: String) -> Self {
        Self {
            user_repo,
            jwt_secret,
        }
    }

    /// Register a new user from the provided registration data.
    ///
    /// Validates the `RegisterUserDto`, stores a hashed password, and returns the created user model.
    ///
    /// # Returns
    ///
    /// The created `user::Model`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::sync::Arc;
    /// # use crates_core::service::UserService;
    /// # use crates_core::dto::RegisterUserDto;
    /// # use crates_core::repository::InMemoryUserRepo;
    /// // Construct service with a repository and call register.
    /// let repo = Arc::new(InMemoryUserRepo::default());
    /// let svc = UserService::new(repo, "secret".to_string());
    /// let dto = RegisterUserDto {
    ///     user_name: "alice".to_string(),
    ///     email: "alice@example.com".to_string(),
    ///     password: "s3cret".to_string(),
    ///     first_name: Some("Alice".to_string()),
    ///     last_name: Some("Example".to_string()),
    ///     address: None,
    /// };
    /// let created = tokio::runtime::Runtime::new().unwrap().block_on(async {
    ///     svc.register(dto).await.unwrap()
    /// });
    /// assert_eq!(created.user_name, "alice");
    /// ```
    pub async fn register(&self, dto: RegisterUserDto) -> Result<user::Model, Error> {
        dto.validate()?;

        let RegisterUserDto {
            user_name,
            email,
            password,
            first_name,
            last_name,
            address,
            ..
        } = dto;

        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

        self.user_repo
            .create_new(
                user_name,
                email,
                password_hash,
                first_name,
                last_name,
                address,
            )
            .await
    }

    /// Attempts to authenticate a user with the provided credentials and returns an authentication token on success.
    ///
    /// Returns an unauthorized `Error` when the email exists but the password does not match. Other errors from the user
    /// repository or password verification are propagated.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::executor::block_on;
    /// // let service = /* UserService instance */;
    /// // let dto = /* LoginDto with email and password */;
    /// // let token = block_on(service.login(dto)).unwrap();
    /// // assert!(!token.is_empty());
    /// ```
    pub async fn login(&self, dto: LoginDto) -> Result<String, Error> {
        let user = self.user_repo.get_by_email(dto.email).await?;

        let valid = bcrypt::verify(dto.password, &user.password_hash)?;

        if !valid {
            return Err(Error::unauthorized("Invalid credentials".to_string()));
        }

        if self.jwt_secret.is_empty() {
            return Err(Error::internal("JWT secret is not configured".to_string()));
        }

        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::hours(24)).timestamp() as usize;

        let claims = Claims {
            sub: user.id,
            iat,
            exp,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|e| Error::internal(format!("Failed to create JWT token: {}", e)))?;

        Ok(token)
    }
}
