#[derive(Debug, PartialEq, Eq)]
pub enum HasherError {
    HashingError { message: String },
    VerificationError { message: String },
}

impl std::fmt::Display for HasherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HashingError { message } => {
                write!(f, "an error occurred while hashing password: {message}")
            }
            Self::VerificationError { message } => {
                write!(f, "an error occurred while verifying password: {message}")
            }
        }
    }
}

impl std::error::Error for HasherError {}

pub trait HasherPort: Send + Sync {
    /// Hashes a password string.
    ///
    /// # Errors
    ///
    /// Returns a `HasherError` if hashing fails for any reason.
    fn hash(&self, password: &str) -> Result<String, HasherError>;
    /// Verifies a password against a hashed password.
    ///
    /// # Errors
    ///
    /// Returns `HasherError` if the verification fails.
    fn verify(&self, password: &str, password_hash: &str) -> Result<bool, HasherError>;
}
