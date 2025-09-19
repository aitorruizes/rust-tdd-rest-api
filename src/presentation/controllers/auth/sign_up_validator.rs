use serde_json::{Value, json};

#[derive(Clone)]
pub struct SignUpValidator;

impl SignUpValidator {
    pub fn new() -> Self {
        SignUpValidator
    }

    pub fn validate(&self, fields: serde_json::Value) -> Result<(), serde_json::Value> {
        let mut errors: Vec<Value> = Vec::new();

        match fields.get("first_name") {
            Some(value) => {
                if let Some(s) = value.as_str() {
                    if s.trim().is_empty() {
                        errors.push(json!({"field": "first_name", "error": "empty"}));
                    }
                } else {
                    errors.push(json!({"field": "first_name", "expected_type": "string"}));
                }
            }
            None => errors.push(json!({"field": "first_name", "error": "missing"})),
        }

        match fields.get("last_name") {
            Some(value) => {
                if let Some(s) = value.as_str() {
                    if s.trim().is_empty() {
                        errors.push(json!({"field": "last_name", "error": "empty"}));
                    }
                } else {
                    errors.push(json!({"field": "last_name", "expected_type": "string"}));
                }
            }
            None => errors.push(json!({"field": "last_name", "error": "missing"})),
        }

        match fields.get("email") {
            Some(value) => {
                if let Some(s) = value.as_str() {
                    if s.trim().is_empty() {
                        errors.push(json!({"field": "email", "error": "empty"}));
                    }
                } else {
                    errors.push(json!({"field": "email", "expected_type": "string"}));
                }
            }
            None => errors.push(json!({"field": "email", "error": "missing"})),
        }

        match fields.get("password") {
            Some(value) => {
                if let Some(s) = value.as_str() {
                    if s.trim().is_empty() {
                        errors.push(json!({"field": "password", "error": "empty"}));
                    }
                } else {
                    errors.push(json!({"field": "password", "expected_type": "string"}));
                }
            }
            None => errors.push(json!({"field": "password", "error": "missing"})),
        }

        match fields.get("password_confirmation") {
            Some(value) => {
                if let Some(s) = value.as_str() {
                    if s.trim().is_empty() {
                        errors.push(json!({"field": "password_confirmation", "error": "empty"}));
                    }
                } else {
                    errors
                        .push(json!({"field": "password_confirmation", "expected_type": "string"}));
                }
            }
            None => errors.push(json!({"field": "password_confirmation", "error": "missing"})),
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
