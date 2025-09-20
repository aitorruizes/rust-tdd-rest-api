use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum AuthError {
    GenerateTokenError { message: String },
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::GenerateTokenError { message } => {
                write!(
                    f,
                    "an error occurred while generation auth token: {}",
                    message
                )
            }
        }
    }
}

impl std::error::Error for AuthError {}

pub trait AuthPort: AuthPortClone + Send + Sync {
    fn generate_auth_token(&self, user_id: Uuid) -> Result<String, AuthError>;
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
