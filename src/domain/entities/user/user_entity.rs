#[derive(Debug, Clone, PartialEq)]
pub struct UserEntity {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct UserEntityBuilder {
    id: String,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    is_admin: bool,
    created_at: String,
    updated_at: String,
}

impl UserEntityBuilder {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            is_admin: false,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn first_name(mut self, first_name: impl Into<String>) -> Self {
        self.first_name = first_name.into();
        self
    }

    pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = last_name.into();
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();
        self
    }

    pub fn is_admin(mut self, is_admin: bool) -> Self {
        self.is_admin = is_admin;
        self
    }

    pub fn created_at(mut self, created_at: impl Into<String>) -> Self {
        self.created_at = created_at.into();
        self
    }

    pub fn updated_at(mut self, updated_at: impl Into<String>) -> Self {
        self.updated_at = updated_at.into();
        self
    }

    pub fn build(self) -> UserEntity {
        UserEntity {
            id: self.id,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            password: self.password,
            is_admin: self.is_admin,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl Default for UserEntityBuilder {
    fn default() -> Self {
        Self::new()
    }
}
