use app_core::dto::auth::TokenClaims;
use axum::{
    Json,
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use serde::Serialize;

use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug)]
pub struct JwtAuth(pub TokenClaims);

impl<S> FromRequestParts<S> for JwtAuth
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = (StatusCode, Json<ErrorResponse>);

    /// Extracts JWT claims from the request's `Authorization: Bearer` header and returns them wrapped in `JwtAuth`.
    ///
    /// On success returns `JwtAuth` containing the parsed `TokenClaims`. On failure returns a `401 Unauthorized` rejection with a JSON `ErrorResponse`:
    /// - missing or unparsable `Authorization` header => `ErrorResponse { status: "fail", message: "Authorization header missing" }`
    /// - invalid token => `ErrorResponse { status: "fail", message: "Invalid JWT token: <error>" }`
    ///
    /// # Examples
    ///
    /// ```
    /// use axum::response::IntoResponse;
    /// use axum::routing::get;
    /// use axum::{Router, Json};
    ///
    /// // Example handler that uses the extractor
    /// async fn protected_route(JwtAuth(claims): JwtAuth) -> impl IntoResponse {
    ///     // Use `claims` to authorize or customize response
    ///     Json(format!("user id: {}", claims.sub))
    /// }
    ///
    /// // In router setup:
    /// // let app = Router::new().route("/protected", get(protected_route));
    /// ```
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let auth_service = &app_state.auth_service;

        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(ErrorResponse {
                            status: "fail".to_string(),
                            message: "Authorization header missing".to_string(),
                        }),
                    )
                })?;

        let claims = auth_service.verify_token(bearer.token()).map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    status: "fail".to_string(),
                    message: format!("Invalid JWT token: {}", e),
                }),
            )
        })?;

        Ok(JwtAuth(claims))
    }
}
