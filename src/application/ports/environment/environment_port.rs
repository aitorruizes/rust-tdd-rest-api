#[derive(Debug)]
pub enum EnvironmentError {
    FileNotLoaded,
    VariableNotFound { key: String },
}

impl std::fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvironmentError::FileNotLoaded => {
                write!(f, "Environment file not found.")
            }
            EnvironmentError::VariableNotFound { key } => {
                write!(f, "Environment variable '{}' not found.", key)
            }
        }
    }
}

impl std::error::Error for EnvironmentError {}

pub trait EnvironmentPort {
    fn load_environment_file(&self) -> Result<(), EnvironmentError>;
    fn get_environment_file(&self, key: &str) -> Result<String, EnvironmentError>;
}
