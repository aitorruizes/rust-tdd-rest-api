pub trait IdGeneratorPort: IdGeneratorPortClone + Send + Sync {
    fn generate_id(&self) -> String;
}

pub trait IdGeneratorPortClone {
    fn clone_box(&self) -> Box<dyn IdGeneratorPort + Send + Sync>;
}

impl<T> IdGeneratorPortClone for T
where
    T: IdGeneratorPort + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn IdGeneratorPort + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn IdGeneratorPort + Send + Sync> {
    fn clone(&self) -> Box<dyn IdGeneratorPort + Send + Sync> {
        self.as_ref().clone_box()
    }
}
