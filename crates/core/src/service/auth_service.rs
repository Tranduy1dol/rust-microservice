use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{dto::auth::TokenClaims, error::Error};

pub struct AuthService {
    jwt_secret: String,
}

impl AuthService {
    /// Creates a new AuthService configured with the provided JWT secret.
    ///
    /// # Examples
    ///
    /// ```
    /// let svc = AuthService::new("my_jwt_secret".to_string());
    /// // use `svc` to verify tokens: `svc.verify_token(token_str)`
    /// ```
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    /// Verify a JWT and return its decoded claims if the token is valid.
    ///
    /// # Examples
    ///
    /// ```
    /// let svc = AuthService::new("my-secret".to_string());
    /// // An invalid or malformed token will yield an error.
    /// assert!(svc.verify_token("invalid.token").is_err());
    /// ```
    ///
    /// # Returns
    ///
    /// `TokenClaims` parsed from the provided token on success; `Error::unauthorized` if the token is invalid or fails validation.
    pub fn verify_token(&self, token: &str) -> Result<TokenClaims, Error> {
        let validation = Validation::default();
        let token_data = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        )
        .map_err(|e| Error::unauthorized(format!("Invalid JWT token: {}", e)))?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};

    /// Generates a JWT containing the given subject (`sub`) with issued-at and expiration claims.
    ///
    /// The token's `iat` is set to the current time and `exp` is set to the current time plus
    /// `exp_in_hours` hours. The token is encoded using `secret` and returned as a compact JWT string.
    ///
    /// Panics if encoding the token fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let token = generate_token("my-secret", 123, 1);
    /// assert!(!token.is_empty());
    /// ```
    fn generate_token(secret: &str, sub: i64, exp_in_hours: i64) -> String {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::hours(exp_in_hours)).timestamp() as usize;

        let claims = TokenClaims { sub, iat, exp };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap()
    }

    #[test]
    fn test_verify_valid_token() {
        let secret = "test_secret";
        let auth_service = AuthService::new(secret.to_string());
        let token = generate_token(secret, 123, 1);

        let result = auth_service.verify_token(&token);
        assert!(result.is_ok());
        let claims = result.unwrap();
        assert_eq!(claims.sub, 123);
    }

    #[test]
    fn test_verify_expired_token() {
        let secret = "test_secret";
        let auth_service = AuthService::new(secret.to_string());
        // Generate token expired 1 hour ago
        let token = generate_token(secret, 123, -1);

        let result = auth_service.verify_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_invalid_signature() {
        let secret = "test_secret";
        let auth_service = AuthService::new(secret.to_string());
        let token = generate_token("wrong_secret", 123, 1);

        let result = auth_service.verify_token(&token);
        assert!(result.is_err());
    }
}
