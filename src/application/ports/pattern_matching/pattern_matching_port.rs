#[derive(Debug)]
pub enum PatternMatchingError {
    InvalidRegex,
    InvalidEmail,
    InvalidEmailDomain,
    InvalidPassword,
}

impl std::fmt::Display for PatternMatchingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternMatchingError::InvalidRegex => {
                write!(f, "the provided regex is invalid")
            }
            PatternMatchingError::InvalidEmail => {
                write!(f, "the provided e-mail is invalid")
            }
            PatternMatchingError::InvalidEmailDomain => {
                write!(f, "the provided e-mail's domain is blacklisted")
            }
            PatternMatchingError::InvalidPassword => {
                write!(
                    f,
                    "the provided password should contain at least 12 characters"
                )
            }
        }
    }
}

impl std::error::Error for PatternMatchingError {}

pub trait PatternMatchingPort: PatternMatchingPortClone + Send + Sync {
    fn is_valid_email(&self, email: &str) -> Result<bool, PatternMatchingError>;
    fn is_valid_email_domain(&self, email: &str) -> Result<bool, PatternMatchingError>;
    fn is_valid_password(&self, password: &str) -> Result<bool, PatternMatchingError>;
}

pub trait PatternMatchingPortClone {
    fn clone_box(&self) -> Box<dyn PatternMatchingPort + Send + Sync>;
}

impl<T> PatternMatchingPortClone for T
where
    T: PatternMatchingPort + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn PatternMatchingPort + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn PatternMatchingPort + Send + Sync> {
    fn clone(&self) -> Box<dyn PatternMatchingPort + Send + Sync> {
        self.as_ref().clone_box()
    }
}
