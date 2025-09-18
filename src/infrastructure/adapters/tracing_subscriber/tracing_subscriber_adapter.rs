use crate::application::ports::logger_subscriber::logger_subsriber_port::LoggerSubscriberPort;

pub struct TracingSubscriberAdapter;

impl TracingSubscriberAdapter {
    pub fn new() -> Self {
        TracingSubscriberAdapter
    }
}

impl LoggerSubscriberPort for TracingSubscriberAdapter {
    fn initialize(self) {
        tracing_subscriber::fmt().init()
    }
}

impl Default for TracingSubscriberAdapter {
    fn default() -> Self {
        Self::new()
    }
}
