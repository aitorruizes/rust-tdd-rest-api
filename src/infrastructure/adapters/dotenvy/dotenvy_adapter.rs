use crate::application::ports::environment::environment_port::{EnvironmentError, EnvironmentPort};

pub struct DotenvyAdapter {
    is_environment_file_load: bool,
}

impl DotenvyAdapter {
    pub fn new() -> Self {
        DotenvyAdapter {
            is_environment_file_load: false,
        }
    }
}

impl EnvironmentPort for DotenvyAdapter {
    fn load_environment_file(&self) -> Result<(), EnvironmentError> {
        if !self.is_environment_file_load {
            dotenvy::dotenv().map_err(|_| EnvironmentError::FileNotLoaded)?;
        }

        Ok(())
    }

    fn get_environment_file(&self, key: &str) -> Result<String, EnvironmentError> {
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
