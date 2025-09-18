pub trait LoggerPort {
    fn log_info(&self, message: &str);
    fn log_error(&self, message: &str);
    fn log_warn(&self, message: &str);
}
