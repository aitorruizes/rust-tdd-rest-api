use crate::infrastructure::boostrap::api::api_boostrap::{ApiBootstrap, ApiBootstrapPort};

pub mod domain {
    pub mod entities {
        pub mod user {
            pub mod user_entity;
        }
    }

    pub mod errors {
        pub mod user {
            pub mod user_errors;
        }
    }
}

pub mod application {
    pub mod ports {
        pub mod database {
            pub mod database_port;
            pub mod user_database_port;
        }

        pub mod hasher {
            pub mod hasher_port;
        }

        pub mod id_generator {
            pub mod id_generator_port;
        }

        pub mod logger {
            pub mod logger_port;
        }

        pub mod logger_subscriber {
            pub mod logger_subsriber_port;
        }

        pub mod tcp_server {
            pub mod tcp_server_port;
        }

        pub mod environment {
            pub mod environment_port;
        }

        pub mod web_framework {
            pub mod web_framework_port;
        }

        pub mod validator {
            pub mod validator_port;
        }

        pub mod pattern_matching {
            pub mod pattern_matching_port;
        }
    }

    pub mod use_cases {
        pub mod user {
            pub mod create_user_use_case;
        }
    }
}

pub mod infrastructure {
    pub mod repositories {
        pub mod user {
            pub mod create_user_repository;
        }
    }

    pub mod adapters {
        pub mod bcrypt {
            pub mod bcrypt_adapter;
        }

        pub mod uuid {
            pub mod uuid_adapter;
        }

        pub mod tracing {
            pub mod tracing_adapter;
        }

        pub mod tracing_subscriber {
            pub mod tracing_subscriber_adapter;
        }

        pub mod tokio {
            pub mod tokio_adapter;
        }

        pub mod dotenvy {
            pub mod dotenvy_adapter;
        }

        pub mod axum {
            pub mod axum_adapter;
            pub mod axum_route_adapter;
        }

        pub mod regex {
            pub mod regex_adapter;
        }
    }

    pub mod gateways {
        pub mod database {
            pub mod database_gateway;
            pub mod user_database_gateway;
        }
    }

    pub mod boostrap {
        pub mod api {
            pub mod api_boostrap;
        }
    }
}

pub mod presentation {
    pub mod dtos {
        pub mod user {
            pub mod create_user_dto;
        }

        pub mod http {
            pub mod http_request_dto;
            pub mod http_response_dto;
        }
    }

    pub mod controllers {
        pub mod user {
            pub mod create_user_controller;
            pub mod create_user_validator;
        }
    }

    pub mod ports {
        pub mod controller {
            pub mod controller_port;
        }

        pub mod router {
            pub mod router_port;
        }
    }

    pub mod routers {
        pub mod auth {
            pub mod auth_router;
        }

        pub mod core {
            pub mod core_router;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_boostrap: ApiBootstrap = ApiBootstrap;

    api_boostrap.setup().await
}
