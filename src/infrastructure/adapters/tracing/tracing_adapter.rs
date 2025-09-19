pub struct TracingAdapter;

impl TracingAdapter {
    pub fn new() -> Self {
        TracingAdapter
    }

    pub fn log_info(&self, message: &str) {
        tracing::info!("{}", message);
    }

    pub fn log_error(&self, message: &str) {
        tracing::error!("{}", message);
    }

    pub fn log_warn(&self, message: &str) {
        tracing::warn!("{}", message);
    }
}

impl Default for TracingAdapter {
    fn default() -> Self {
        Self::new()
    }
}
