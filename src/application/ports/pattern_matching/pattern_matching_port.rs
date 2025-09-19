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
                    "the provided password should contain at least 8 characters, including at least one uppercase letter, one lowercase letter, one number, and one special character (@, $, !, %, *, ?, &)"
                )
            }
        }
    }
}

impl std::error::Error for PatternMatchingError {}

pub trait PatternMatchingPort {
    fn is_valid_email(&self, email: &str) -> Result<bool, PatternMatchingError>;
    fn is_valid_email_domain(&self, email: &str) -> Result<bool, PatternMatchingError>;
    fn is_valid_password(&self, password: &str) -> Result<bool, PatternMatchingError>;
}
