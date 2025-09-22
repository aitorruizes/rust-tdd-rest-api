use regex::Regex;

use crate::application::ports::pattern_matching::pattern_matching_port::{
    PatternMatchingError, PatternMatchingPort,
};

#[derive(Clone)]
pub struct RegexAdapter;

impl RegexAdapter {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl PatternMatchingPort for RegexAdapter {
    fn is_valid_email(&self, email: &str) -> Result<bool, PatternMatchingError> {
        let regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .map_err(|_| PatternMatchingError::InvalidRegex)?;

        Ok(regex.is_match(email))
    }

    fn is_valid_email_domain(&self, email: &str) -> Result<bool, PatternMatchingError> {
        let regex = Regex::new(
            r"^[a-zA-Z0-9._%+-]+@(gmail\.com|outlook\.com|yahoo\.com|hotmail\.com|live\.com)$",
        )
        .map_err(|_| PatternMatchingError::InvalidRegex)?;

        Ok(regex.is_match(email))
    }

    fn is_valid_password(&self, password: &str) -> Result<bool, PatternMatchingError> {
        let regex = Regex::new(r"^[A-Za-z\d!@#$%^&*]{12,}$")
            .map_err(|_| PatternMatchingError::InvalidRegex)?;

        Ok(regex.is_match(password))
    }

    fn is_valid_uuid(&self, uuid: &str) -> Result<bool, PatternMatchingError> {
        let regex = Regex::new(
            r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
        )
        .map_err(|_| PatternMatchingError::InvalidRegex)?;

        Ok(regex.is_match(uuid))
    }
}

impl Default for RegexAdapter {
    fn default() -> Self {
        Self::new()
    }
}
