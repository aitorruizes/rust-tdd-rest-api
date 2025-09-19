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

pub struct DotenvyAdapter {
    is_environment_file_load: bool,
}

impl DotenvyAdapter {
    pub fn new() -> Self {
        DotenvyAdapter {
            is_environment_file_load: false,
        }
    }

    pub fn load_environment_file(&self) -> Result<(), EnvironmentError> {
        if !self.is_environment_file_load {
            dotenvy::dotenv().map_err(|_| EnvironmentError::FileNotLoaded)?;
        }

        Ok(())
    }

    pub fn get_environment_file(&self, key: &str) -> Result<String, EnvironmentError> {
        let environment_variable: String =
            std::env::var(key).map_err(|_| EnvironmentError::VariableNotFound {
                key: key.to_string(),
            })?;

        Ok(environment_variable)
    }
}

impl Default for DotenvyAdapter {
    fn default() -> Self {
        Self::new()
    }
}
