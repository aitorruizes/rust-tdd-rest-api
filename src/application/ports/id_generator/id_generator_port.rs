pub trait IdGeneratorPort {
    fn generate_id(&self) -> String;
}
