#[derive(Debug)]
pub enum RegexError {
    InvalidRegex,
    InvalidEmail,
    InvalidEmailDomain,
    InvalidPassword,
}

impl std::fmt::Display for RegexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegexError::InvalidRegex => {
                write!(f, "the provided regex is invalid")
            }
            RegexError::InvalidEmail => {
                write!(f, "the provided e-mail is invalid")
            }
            RegexError::InvalidEmailDomain => {
                write!(f, "the provided e-mail's domain is blacklisted")
            }
            RegexError::InvalidPassword => {
                write!(
                    f,
                    "the provided password should contain at least 12 characters"
                )
            }
        }
    }
}

impl std::error::Error for RegexError {}

pub trait PatternMatchingPort: Send + Sync {
    fn is_valid_email(&self, email: &str) -> Result<bool, RegexError>;
    fn is_valid_email_domain(&self, email: &str) -> Result<bool, RegexError>;
    fn is_valid_password(&self, password: &str) -> Result<bool, RegexError>;
}
