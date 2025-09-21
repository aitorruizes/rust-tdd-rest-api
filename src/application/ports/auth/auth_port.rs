use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
pub enum AuthError {
    GenerateTokenError { message: String },
    InvalidTokenError,
    ExpiredTokenError,
    UnexpectedError,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GenerateTokenError { message } => {
                write!(
                    f,
                    "an error occurred while generating authorization token: {message}",
                )
            }
            Self::InvalidTokenError => {
                write!(f, "the provided authorization token is invalid")
            }
            Self::ExpiredTokenError => {
                write!(f, "the provided authorization token has expired")
            }
            Self::UnexpectedError => {
                write!(f, "an unexpected error has occurred")
            }
        }
    }
}

impl std::error::Error for AuthError {}

pub trait AuthPort: Send + Sync {
    /// Generates an authentication token for the given user ID.
    ///
    /// # Errors
    ///
    /// Returns `AuthError` if the token generation fails for any reason.
    fn generate_auth_token(&self, user_id: Uuid) -> Result<String, AuthError>;

    /// Verifies an authentication token.
    ///
    /// # Errors
    ///
    /// Returns `AuthError` if the token is invalid, expired, or cannot be verified.
    fn verify_auth_token(&self, token: &str) -> Result<(), AuthError>;
}
