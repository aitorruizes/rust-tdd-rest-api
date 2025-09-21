use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SignInDto {
    pub email: String,
    pub password: String,
}

impl SignInDto {
    #[must_use]
    pub const fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}
