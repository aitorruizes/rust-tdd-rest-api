use regex::Regex;

use crate::application::ports::pattern_matching::pattern_matching_port::{
    PatternMatchingPort, RegexError,
};

#[derive(Clone)]
pub struct RegexAdapter;

impl RegexAdapter {
    pub fn new() -> Self {
        RegexAdapter
    }
}

impl PatternMatchingPort for RegexAdapter {
    fn is_valid_email(&self, email: &str) -> Result<bool, RegexError> {
        let regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .map_err(|_| RegexError::InvalidRegex)?;

        Ok(regex.is_match(email))
    }

    fn is_valid_email_domain(&self, email: &str) -> Result<bool, RegexError> {
        let regex = Regex::new(
            r"^[a-zA-Z0-9._%+-]+@(gmail\.com|outlook\.com|yahoo\.com|hotmail\.com|live\.com)$",
        )
        .map_err(|_| RegexError::InvalidRegex)?;

        Ok(regex.is_match(email))
    }

    fn is_valid_password(&self, password: &str) -> Result<bool, RegexError> {
        let regex =
            Regex::new(r"^[A-Za-z\d!@#$%^&*]{12,}$").map_err(|_| RegexError::InvalidRegex)?;

        Ok(regex.is_match(password))
    }
}

impl Default for RegexAdapter {
    fn default() -> Self {
        Self::new()
    }
}
