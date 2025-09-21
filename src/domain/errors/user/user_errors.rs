#[derive(Debug, PartialEq, Eq)]
pub enum UserError {
    PasswordsDoNotMatch,
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PasswordsDoNotMatch => write!(f, "the provided passwords do not match"),
        }
    }
}

impl std::error::Error for UserError {}
