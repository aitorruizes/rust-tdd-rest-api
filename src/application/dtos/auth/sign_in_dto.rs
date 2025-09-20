use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SignInDto {
    pub email: String,
    pub password: String,
}

impl SignInDto {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}
