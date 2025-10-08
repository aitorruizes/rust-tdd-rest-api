use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::application::ports::auth::auth_port::{AuthError, AuthPort};

#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::cast_possible_truncation)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Clone)]
pub struct JsonWebTokenAdapter;

impl JsonWebTokenAdapter {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl AuthPort for JsonWebTokenAdapter {
    fn generate_auth_token(&self, user_id: &str) -> Result<String, AuthError> {
        let expiration = usize::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 120,
        )
        .map_err(|_| AuthError::UnexpectedError)?;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
        };

        let auth_token = encode(
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
            _ => {
                println!("{err}");
                AuthError::UnexpectedError
            }
        })
    }
}

impl Default for JsonWebTokenAdapter {
    fn default() -> Self {
        Self::new()
    }
}
