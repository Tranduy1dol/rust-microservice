use std::sync::Arc;

use chrono::{Duration, Utc};
use entities::user;
use jsonwebtoken::{encode, EncodingKey, Header};
use tokio::sync::mpsc;
use validator::Validate;

use crate::{
    dto::{
        auth::TokenClaims,
        user_dto::{LoginDto, RegisterUserDto},
    },
    error::Error,
    events::AppEvent,
    ports::user_repo::UserRepository,
};

pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
    event_sender: mpsc::Sender<AppEvent>,
    jwt_secret: String,
}

impl UserService {
    /// Create a UserService backed by the provided repository, JWT signing secret, and event sender.
    ///
    /// The returned service will use `user_repo` for persistence, `jwt_secret` to sign tokens, and `event_sender` to publish application events.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::sync::Arc;
    /// use tokio::sync::mpsc;
    /// # use crates_core::service::UserService;
    /// # use crates_core::repository::InMemoryUserRepo;
    /// # use crates_core::events::AppEvent;
    ///
    /// let repo = Arc::new(InMemoryUserRepo::default());
    /// let jwt_secret = "your-jwt-secret".to_string();
    /// let (tx, _rx) = mpsc::channel::<AppEvent>(8);
    /// let svc = UserService::new(repo, jwt_secret, tx);
    /// ```
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        jwt_secret: String,
        event_sender: mpsc::Sender<AppEvent>,
    ) -> Self {
        Self {
            user_repo,
            jwt_secret,
            event_sender,
        }
    }

    /// Create a new user and publish a `AppEvent::UserRegistered` event.
    ///
    /// Validates the provided `RegisterUserDto`, persists a new user in the repository,
    /// and attempts to publish a `AppEvent::UserRegistered` using the service's event sender.
    ///
    /// # Returns
    ///
    /// The created `user::Model`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::sync::Arc;
    /// use tokio::sync::mpsc;
    /// use crates_core::service::UserService;
    /// use crates_core::dto::RegisterUserDto;
    /// use crates_core::repository::InMemoryUserRepo;
    ///
    /// let repo = Arc::new(InMemoryUserRepo::default());
    /// let (tx, _rx) = mpsc::channel(16);
    /// let svc = UserService::new(repo, "secret".to_string(), tx);
    ///
    /// let dto = RegisterUserDto {
    ///     user_name: "alice".to_string(),
    ///     email: "alice@example.com".to_string(),
    ///     password: "s3cret".to_string(),
    ///     first_name: Some("Alice".to_string()),
    ///     last_name: Some("Example".to_string()),
    ///     address: None,
    /// };
    ///
    /// let created = tokio::runtime::Runtime::new().unwrap().block_on(async {
    ///     svc.register(dto).await.unwrap()
    /// });
    ///
    /// assert_eq!(created.user_name, "alice");
    /// ```
    #[tracing::instrument(skip_all, fields(user_email = %dto.email))]
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

        let user = self
            .user_repo
            .create_new(
                user_name,
                email,
                password_hash,
                first_name,
                last_name,
                address,
            )
            .await?;

        let event = AppEvent::UserRegistered {
            user_id: user.id,
            email: user.email.clone(),
        };

        if let Err(e) = self.event_sender.try_send(event) {
            tracing::error!("Failed to send UserRegistered event: {}", e);
        }

        Ok(user)
    }

    /// Authenticates a user and returns a signed JWT for the authenticated user.
    ///
    /// Returns the JWT token string on success. Returns an unauthorized `Error` when the email exists but the password does not match. Returns an internal `Error` if the JWT secret is not configured or if token creation fails. Other errors from the user repository or password verification are propagated.
    ///
    /// # Examples
    ///
    /// ```ignore
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

        let claims = TokenClaims {
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