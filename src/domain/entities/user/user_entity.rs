use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct UserEntity {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub struct UserEntityBuilder {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    is_admin: bool,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl UserEntityBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: Uuid::nil(),
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            password: String::new(),
            is_admin: false,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }

    #[must_use]
    pub fn id(mut self, id: impl Into<Uuid>) -> Self {
        self.id = id.into();
        self
    }

    #[must_use]
    pub fn first_name(mut self, first_name: impl Into<String>) -> Self {
        self.first_name = first_name.into();
        self
    }

    #[must_use]
    pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = last_name.into();
        self
    }

    #[must_use]
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    #[must_use]
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();
        self
    }

    #[must_use]
    pub const fn is_admin(mut self, is_admin: bool) -> Self {
        self.is_admin = is_admin;
        self
    }

    #[must_use]
    pub fn created_at(mut self, created_at: impl Into<OffsetDateTime>) -> Self {
        self.created_at = created_at.into();
        self
    }

    #[must_use]
    pub fn updated_at(mut self, updated_at: impl Into<OffsetDateTime>) -> Self {
        self.updated_at = updated_at.into();
        self
    }

    #[must_use]
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
