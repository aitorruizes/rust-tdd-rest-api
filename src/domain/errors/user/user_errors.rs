#[derive(Debug, PartialEq)]
pub enum UserError {
    PasswordsDoNotMatch,
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::PasswordsDoNotMatch => write!(f, "The provided passwords do not match."),
        }
    }
}

impl std::error::Error for UserError {}
