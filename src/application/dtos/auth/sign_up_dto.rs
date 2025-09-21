use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SignUpDto {
    pub id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl SignUpDto {
    #[must_use]
    pub const fn new(
        first_name: String,
        last_name: String,
        email: String,
        password: String,
    ) -> Self {
        Self {
            id: None,
            first_name,
            last_name,
            email,
            password,
        }
    }
}
