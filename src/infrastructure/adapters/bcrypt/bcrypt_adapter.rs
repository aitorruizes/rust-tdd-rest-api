use crate::application::ports::hasher::hasher_port::{HasherError, HasherPort};

pub struct BcryptAdapter;

impl BcryptAdapter {
    pub fn new() -> Self {
        BcryptAdapter {}
    }
}

impl HasherPort for BcryptAdapter {
    fn hash(&self, password: &str) -> Result<String, HasherError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|err| HasherError::HashingError {
            message: err.to_string(),
        })
    }

    fn verify(&self, password: &str, password_hash: &str) -> Result<bool, HasherError> {
        bcrypt::verify(password, password_hash).map_err(|err| HasherError::VerificationError {
            message: err.to_string(),
        })
    }
}

impl Default for BcryptAdapter {
    fn default() -> Self {
        Self::new()
    }
}
