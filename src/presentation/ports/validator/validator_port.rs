use serde_json::Value;

pub trait ValidatorPort {
    /// Validates the given JSON `fields`.
    ///
    /// # Parameters
    /// - `fields`: Reference to a `serde_json::Value` representing the data to validate.
    ///
    /// # Returns
    /// - `Ok(())` if the validation succeeds.
    ///
    /// # Errors
    /// Returns `Err(Value)` if validation fails. The `Value` contains details about the
    /// validation errors, such as missing fields, incorrect types, or invalid values.
    ///
    /// # Panics
    /// This function should not panic. All validation errors must be returned via `Err`.
    fn validate(&self, fields: &Value) -> Result<(), Value>;
}
