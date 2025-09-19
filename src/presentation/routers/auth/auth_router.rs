use crate::{
    application::ports::web_framework::web_framework_port::{
        RouterMethod, RouterWrapper, WebFrameworkRoutePort,
    },
    presentation::ports::{
        controller::controller_port::ControllerPort, router::router_port::RouterPort,
    },
};

pub struct AuthRouter {
    web_framework_route_adapter: Box<dyn WebFrameworkRoutePort>,
    create_user_controller: Box<dyn ControllerPort>,
}

impl AuthRouter {
    pub fn new(
        web_framework_route_adapter: Box<dyn WebFrameworkRoutePort>,
        create_user_controller: Box<dyn ControllerPort>,
    ) -> Self {
        AuthRouter {
            web_framework_route_adapter,
            create_user_controller,
        }
    }
}

impl RouterPort for AuthRouter {
    fn register_routes(self) -> Box<dyn RouterWrapper> {
        self.web_framework_route_adapter.create_router(
            RouterMethod::Post,
            "/auth/sign-up",
            self.create_user_controller,
        )
    }
}
