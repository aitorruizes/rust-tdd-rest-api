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
            Self::InvalidRegex => {
                write!(f, "the provided regex is invalid")
            }
            Self::InvalidEmail => {
                write!(f, "the provided e-mail is invalid")
            }
            Self::InvalidEmailDomain => {
                write!(f, "the provided e-mail's domain is blacklisted")
            }
            Self::InvalidPassword => {
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
    /// Checks if the provided email is valid.
    ///
    /// # Errors
    ///
    /// Returns `RegexError` if the email cannot be processed by the validation regex.
    fn is_valid_email(&self, email: &str) -> Result<bool, RegexError>;

    /// Checks if the domain part of the provided email is valid.
    ///
    /// # Errors
    ///
    /// Returns `RegexError` if the domain cannot be processed by the validation regex.
    fn is_valid_email_domain(&self, email: &str) -> Result<bool, RegexError>;

    /// Checks if the provided password meets validation requirements.
    ///
    /// # Errors
    ///
    /// Returns `RegexError` if the password cannot be processed by the validation regex.
    fn is_valid_password(&self, password: &str) -> Result<bool, RegexError>;
}
