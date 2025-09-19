use serde_json::Value;

pub trait ValidatorPort: ValidatorPortClone + Send + Sync {
    fn validate(&self, fields: Value) -> Result<(), Value>;
}

pub trait ValidatorPortClone {
    fn clone_box(&self) -> Box<dyn ValidatorPort + Send + Sync>;
}

impl<T> ValidatorPortClone for T
where
    T: ValidatorPort + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn ValidatorPort + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ValidatorPort + Send + Sync> {
    fn clone(&self) -> Box<dyn ValidatorPort + Send + Sync> {
        self.as_ref().clone_box()
    }
}
