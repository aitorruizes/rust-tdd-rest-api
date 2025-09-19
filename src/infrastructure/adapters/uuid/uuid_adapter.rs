use crate::application::ports::id_generator::id_generator_port::IdGeneratorPort;

#[derive(Clone)]
pub struct UuidAdapter;

impl UuidAdapter {
    pub fn new() -> Self {
        UuidAdapter
    }
}

impl IdGeneratorPort for UuidAdapter {
    fn generate_id(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }
}

impl Default for UuidAdapter {
    fn default() -> Self {
        Self::new()
    }
}
