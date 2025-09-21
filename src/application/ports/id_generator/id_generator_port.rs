use uuid::Uuid;

pub trait IdGeneratorPort: Send + Sync {
    fn generate_id(&self) -> Uuid;
}
