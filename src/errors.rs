use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    Internal(#[from] Internal),
}

impl Error {
    fn get_codes(&self) -> (StatusCode, u16) {
        match self {
            Error::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, 500),
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
