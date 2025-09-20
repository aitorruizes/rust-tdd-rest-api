use serde_json::{Value, json};

#[derive(Clone)]
pub struct SignInValidator;

impl SignInValidator {
    pub fn new() -> Self {
        SignInValidator
    }

    pub fn validate(&self, fields: serde_json::Value) -> Result<(), serde_json::Value> {
        let mut errors: Vec<Value> = vec![];

        let required_fields: [&'static str; 5] = [
            "first_name",
            "last_name",
            "email",
            "password",
            "password_confirmation",
        ];

        for &field in &required_fields {
            match fields.get(field) {
                Some(value) => match value.as_str() {
                    Some(s) if s.trim().is_empty() => {
                        errors.push(json!({"field": field, "error": "empty"}))
                    }
                    Some(_) => {}
                    None => errors.push(json!({"field": field, "expected_type": "string"})),
                },
                None => errors.push(json!({"field": field, "error": "missing"})),
            }
        }

        if let (Some(password), Some(password_confirmation)) = (
            fields.get("password").and_then(|v| v.as_str()),
            fields.get("password_confirmation").and_then(|v| v.as_str()),
        ) && password != password_confirmation
        {
            errors.push(
                    json!({"fields": ["password", "password_confirmation"], "error": "passwords do not match"}),
                );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(json!(errors))
        }
    }
}

impl Default for SignInValidator {
    fn default() -> Self {
        Self::new()
    }
}
