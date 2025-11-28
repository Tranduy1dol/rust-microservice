use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{dto::auth::TokenClaims, error::Error};

pub struct AuthService {
    jwt_secret: String,
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

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
