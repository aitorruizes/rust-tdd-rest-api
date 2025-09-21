use uuid::Uuid;

use crate::application::ports::id_generator::id_generator_port::IdGeneratorPort;

#[derive(Clone)]
pub struct UuidAdapter;

impl UuidAdapter {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl IdGeneratorPort for UuidAdapter {
    fn generate_id(&self) -> Uuid {
        uuid::Uuid::new_v4()
    }
}

impl Default for UuidAdapter {
    fn default() -> Self {
        Self::new()
    }
}
