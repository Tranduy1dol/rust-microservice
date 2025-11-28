use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bcrypt::BcryptError;
use sea_orm::DbErr;
use serde_json::json;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    Internal(#[from] Internal),

    #[error("{0}")]
    BadRequest(#[from] BadRequest),

    #[error("{0}")]
    Unauthorized(#[from] Unauthorized),

    #[error("{0}")]
    NotFound(#[from] NotFound),

    #[error("{0}")]
    HashPassword(#[from] BcryptError),

    #[error("{0}")]
    Database(#[from] DbErr),

    #[error("{0}")]
    Jsonwebtoken(#[from] jsonwebtoken::errors::Error),

    #[error("{0}")]
    Validation(#[from] validator::ValidationErrors),
}

impl Error {
    pub fn internal(message: String) -> Self {
        Self::Internal(Internal { message })
    }

    pub fn unauthorized(message: String) -> Self {
        Self::Unauthorized(Unauthorized { message })
    }

    pub fn not_found(message: String) -> Self {
        Self::NotFound(NotFound { message })
    }

    pub fn bad_request(message: String) -> Self {
        Self::BadRequest(BadRequest { message })
    }

    /// Map an `Error` variant to its HTTP status and application-specific numeric error code.
    ///
    /// # Returns
    ///
    /// A tuple `(StatusCode, u16)` where the first element is the HTTP status code and the second is the application-specific error code.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = Error::NotFound("resource".to_string());
    /// let (status, code) = e.get_codes();
    /// assert_eq!(status, StatusCode::NOT_FOUND);
    /// assert_eq!(code, 10004);
    /// ```
    fn get_codes(&self) -> (StatusCode, u16) {
        match self {
            Error::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, 10001),
            Error::BadRequest(_) => (StatusCode::BAD_REQUEST, 10002),
            Error::Unauthorized(_) => (StatusCode::UNAUTHORIZED, 10003),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, 10004),
            Error::HashPassword(_) => (StatusCode::INTERNAL_SERVER_ERROR, 10005),
            Error::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, 10006),
            Error::Jsonwebtoken(_) => (StatusCode::INTERNAL_SERVER_ERROR, 10007),
            Error::Validation(_) => (StatusCode::BAD_REQUEST, 10008),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, _) = self.get_codes();
        let message = self.to_string();
        let body = Json(json!({ "success": false, "message": message }));

        (status_code, body).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Internal error: {message}")]
pub struct Internal {
    pub message: String,
}

#[derive(thiserror::Error, Debug)]
#[error("Bad Request: {message}")]
pub struct BadRequest {
    pub message: String,
}

#[derive(thiserror::Error, Debug)]
#[error("Unauthorized: {message}")]
pub struct Unauthorized {
    pub message: String,
}

#[derive(thiserror::Error, Debug)]
#[error("Not found: {message}")]
pub struct NotFound {
    pub message: String,
}
