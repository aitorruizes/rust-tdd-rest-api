use serde_json::{Value, json};

use crate::presentation::ports::validator::validator_port::ValidatorPort;

#[derive(Clone)]
pub struct SignUpValidator;

impl SignUpValidator {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl ValidatorPort for SignUpValidator {
    fn validate(&self, fields: &Value) -> Result<(), Value> {
        let mut errors = vec![];

        let required_fields = [
            "firstName",
            "lastName",
            "email",
            "password",
            "passwordConfirmation",
        ];

        for &field in &required_fields {
            match fields.get(field) {
                Some(value) => match value.as_str() {
                    Some(s) if s.trim().is_empty() => {
                        errors.push(json!({"field": field, "error": "empty"}));
                    }
                    Some(_) => {}
                    None => errors.push(json!({"field": field, "expected_type": "string"})),
                },
                None => errors.push(json!({"field": field, "error": "missing"})),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(json!(errors))
        }
    }
}

impl Default for SignUpValidator {
    fn default() -> Self {
        Self::new()
    }
}
