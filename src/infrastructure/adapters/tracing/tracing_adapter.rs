use crate::application::ports::logger::logger_port::LoggerPort;

pub struct TracingAdapter;

impl TracingAdapter {
    pub fn new() -> Self {
        TracingAdapter
    }
}

impl LoggerPort for TracingAdapter {
    fn log_info(&self, message: &str) {
        tracing::info!("{}", message);
    }

    fn log_error(&self, message: &str) {
        tracing::error!("{}", message);
    }

    fn log_warn(&self, message: &str) {
        tracing::warn!("{}", message);
    }
}

impl Default for TracingAdapter {
    fn default() -> Self {
        Self::new()
    }
}
