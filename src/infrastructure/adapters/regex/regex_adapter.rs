use regex::Regex;

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

#[derive(Clone)]
pub struct RegexAdapter;

impl RegexAdapter {
    pub fn new() -> Self {
        RegexAdapter
    }

    pub fn is_valid_email(&self, email: &str) -> Result<bool, RegexError> {
        let regex: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .map_err(|_| RegexError::InvalidRegex)?;

        Ok(regex.is_match(email))
    }

    pub fn is_valid_email_domain(&self, email: &str) -> Result<bool, RegexError> {
        let regex: Regex = Regex::new(
            r"^[a-zA-Z0-9._%+-]+@(gmail\.com|outlook\.com|yahoo\.com|hotmail\.com|live\.com)$",
        )
        .map_err(|_| RegexError::InvalidRegex)?;

        Ok(regex.is_match(email))
    }

    pub fn is_valid_password(&self, password: &str) -> Result<bool, RegexError> {
        let regex: Regex =
            Regex::new(r"^[A-Za-z\d!@#$%^&*]{12,}$").map_err(|_| RegexError::InvalidRegex)?;

        Ok(regex.is_match(password))
    }
}

impl Default for RegexAdapter {
    fn default() -> Self {
        Self::new()
    }
}
