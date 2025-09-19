use crate::{
    application::ports::web_framework::web_framework_port::{
        RouterMethod, RouterWrapper, WebFrameworkRoutePort,
    },
    presentation::ports::{
        controller::controller_port::ControllerPort, router::router_port::RouterPort,
    },
};

pub struct CoreRouter {
    web_framework_route_adapter: Box<dyn WebFrameworkRoutePort>,
    create_user_controller: Box<dyn ControllerPort>,
}

impl CoreRouter {
    pub fn new(
        web_framework_route_adapter: Box<dyn WebFrameworkRoutePort>,
        create_user_controller: Box<dyn ControllerPort>,
    ) -> Self {
        CoreRouter {
            web_framework_route_adapter,
            create_user_controller,
        }
    }
}

impl RouterPort for CoreRouter {
    fn register_routes(self) -> Box<dyn RouterWrapper> {
        let auth_router: Box<dyn RouterWrapper> = self.web_framework_route_adapter.create_router(
            RouterMethod::Post,
            "/auth/sign-up",
            self.create_user_controller,
        );

        self.web_framework_route_adapter
            .create_core_router("/api/v1", auth_router)
    }
}
