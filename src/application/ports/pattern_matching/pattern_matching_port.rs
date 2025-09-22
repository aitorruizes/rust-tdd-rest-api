#[derive(Debug)]
pub enum PatternMatchingError {
    InvalidRegex,
    InvalidEmail,
    InvalidEmailDomain,
    InvalidPassword,
    InvalidUuid,
}

impl std::fmt::Display for PatternMatchingError {
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
            Self::InvalidUuid => {
                write!(f, "the provided uuid is invalid")
            }
        }
    }
}

impl std::error::Error for PatternMatchingError {}

pub trait PatternMatchingPort: Send + Sync {
    /// Checks if the provided email is valid.
    ///
    /// # Errors
    ///
    /// Returns `PatternMatchingError` if the email cannot be processed by the validation regex.
    fn is_valid_email(&self, email: &str) -> Result<bool, PatternMatchingError>;

    /// Checks if the domain part of the provided email is valid.
    ///
    /// # Errors
    ///
    /// Returns `PatternMatchingError` if the domain cannot be processed by the validation regex.
    fn is_valid_email_domain(&self, email: &str) -> Result<bool, PatternMatchingError>;

    /// Checks if the provided password meets validation requirements.
    ///
    /// # Errors
    ///
    /// Returns `PatternMatchingError` if the password cannot be processed by the validation regex.
    fn is_valid_password(&self, password: &str) -> Result<bool, PatternMatchingError>;

    /// Checks if the provided UUID meets validation requirements.
    ///
    /// # Errors
    ///
    /// Returns `PatternMatchingError` if the UUUID cannot be processed by the validation regex.
    fn is_valid_uuid(&self, uuid: &str) -> Result<bool, PatternMatchingError>;
}
