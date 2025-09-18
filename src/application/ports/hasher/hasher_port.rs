#[derive(Debug)]
pub enum HasherError {
    HashingError { message: String },
    VerificationError { message: String },
}

impl std::fmt::Display for HasherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HasherError::HashingError { message } => {
                write!(f, "An error occurred while hashing password: {}.", message)
            }
            HasherError::VerificationError { message } => {
                write!(
                    f,
                    "An error occurred while verifying password: {}.",
                    message
                )
            }
        }
    }
}

impl std::error::Error for HasherError {}

pub trait HasherPort {
    fn hash(&self, password: &str) -> Result<String, HasherError>;
    fn verify(&self, password: &str, password_hash: &str) -> Result<bool, HasherError>;
}
