use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum AuthError {
    GenerateTokenError { message: String },
    InvalidTokenError,
    ExpiredTokenError,
    UnexpectedError,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::GenerateTokenError { message } => {
                write!(
                    f,
                    "an error occurred while generating authorization token: {}",
                    message
                )
            }
            AuthError::InvalidTokenError => {
                write!(f, "the provided authorization token is invalid")
            }
            AuthError::ExpiredTokenError => {
                write!(f, "the provided authorization token has expired")
            }
            AuthError::UnexpectedError => {
                write!(f, "an unexpected error has occurred")
            }
        }
    }
}

impl std::error::Error for AuthError {}

pub trait AuthPort: AuthPortClone + Send + Sync {
    fn generate_auth_token(&self, user_id: Uuid) -> Result<String, AuthError>;
    fn verify_auth_token(&self, token: &str) -> Result<(), AuthError>;
}

pub trait AuthPortClone {
    fn clone_box(&self) -> Box<dyn AuthPort + Send + Sync>;
}

impl<T> AuthPortClone for T
where
    T: AuthPort + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn AuthPort + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AuthPort + Send + Sync> {
    fn clone(&self) -> Box<dyn AuthPort + Send + Sync> {
        self.as_ref().clone_box()
    }
}
