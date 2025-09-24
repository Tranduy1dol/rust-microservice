use std::ops::Deref;

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::{JwtConfig, JWT_CONFIG};

const JWT_EXPIRATION_SECONDS: i64 = 30 * 24 * 60 * 60; // 30 days

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenClaims {
    pub iss: String,              // Issuer
    pub sub: i64,                 // Subject (user ID)
    pub email: String,            // User email
    pub name: String,             // User name
    pub aud: String,              // Audience
    pub iat: i64,                 // Issued At
    pub exp: i64,                 // Expiration Time
    pub admin_level: Option<i64>, // Permissions scope
}

pub fn generate_jwt_token(
    user_id: i64,
    email: &str,
    name: &str,
    admin_level: Option<i64>,
    jwt_config: &JwtConfig,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let iat = now.timestamp_millis();
    let exp = (now + Duration::milliseconds(JWT_EXPIRATION_SECONDS)).timestamp_millis();

    let claims = TokenClaims {
        iss: jwt_config.jwt_iss.clone(),
        sub: user_id,
        email: email.to_string(),
        name: name.to_string(),
        aud: jwt_config.jwt_aud[0].clone(),
        iat,
        exp,
        admin_level,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_config.jwt_secret.as_bytes()),
    )
}

pub fn decode_jwt_token(
    token: &str,
    jwt_config: &JwtConfig,
) -> Result<TokenClaims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&jwt_config.jwt_aud);

    decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(jwt_config.jwt_secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth(pub TokenClaims);

impl<S: Send + Sync> FromRequestParts<S> for Auth {
    type Rejection = (StatusCode, Json<String>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json("Authorize header missing".into()),
                    )
                })?;

        let claim = decode_jwt_token(bearer.token(), JWT_CONFIG.deref())
            .map_err(|e| (StatusCode::UNAUTHORIZED, Json(e.to_string())))?;

        Ok(Auth(claim))
    }
}
