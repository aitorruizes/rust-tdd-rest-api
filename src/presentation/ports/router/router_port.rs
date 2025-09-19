use crate::application::ports::web_framework::web_framework_port::RouterWrapper;

pub trait RouterPort {
    fn register_routes(self) -> Box<dyn RouterWrapper>;
}
