use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SignInDto {
    pub email: String,
}

impl SignInDto {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}
