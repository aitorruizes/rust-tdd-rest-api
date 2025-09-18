pub trait IdGeneratorPort: Send + Sync {
    fn generate_id(&self) -> String;
}
