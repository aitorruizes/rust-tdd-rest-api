use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::ports::auth::auth_port::{AuthError, AuthPort};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Clone)]
pub struct JsonWebTokenAdapter;

impl JsonWebTokenAdapter {
    pub fn new() -> Self {
        JsonWebTokenAdapter
    }
}

impl AuthPort for JsonWebTokenAdapter {
    fn generate_auth_token(&self, user_id: Uuid) -> Result<String, AuthError> {
        let expiration: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 5;

        let claims: Claims = Claims {
            sub: user_id.to_string(),
            exp: expiration as usize,
        };

        let auth_token: String = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("asdasdas".as_ref()),
        )
        .map_err(|err| AuthError::GenerateTokenError {
            message: err.to_string(),
        })?;

        Ok(auth_token)
    }

    fn verify_auth_token(&self, token: &str) -> Result<(), AuthError> {
        let mut validation = Validation::new(Algorithm::HS256);

        validation.leeway = 5;
        validation.validate_exp = true;

        decode::<Claims>(
            token,
            &DecodingKey::from_secret("asdasdas".as_ref()),
            &validation,
        )
        .map(|_| ())
        .map_err(|err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::ExpiredTokenError,
            jsonwebtoken::errors::ErrorKind::InvalidToken => AuthError::InvalidTokenError,
            _ => AuthError::UnexpectedError,
        })
    }
}

impl Default for JsonWebTokenAdapter {
    fn default() -> Self {
        Self::new()
    }
}
