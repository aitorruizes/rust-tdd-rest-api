pub struct TracingSubscriberAdapter;

impl TracingSubscriberAdapter {
    pub fn new() -> Self {
        TracingSubscriberAdapter
    }

    pub fn initialize(&self) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init()
    }
}

impl Default for TracingSubscriberAdapter {
    fn default() -> Self {
        Self::new()
    }
}
