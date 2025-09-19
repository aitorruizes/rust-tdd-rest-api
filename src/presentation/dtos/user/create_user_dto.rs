use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CreateUserDto {
    pub id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

impl CreateUserDto {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        password: String,
        password_confirmation: String,
    ) -> Self {
        Self {
            id: None,
            first_name,
            last_name,
            email,
            password,
            password_confirmation,
        }
    }
}
