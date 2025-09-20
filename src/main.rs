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
        pub mod hasher {
            pub mod hasher_port;
        }

        pub mod id_generator {
            pub mod id_generator_port;
        }

        pub mod auth {
            pub mod sign_in_repository_port;
            pub mod sign_up_repository_port;
        }
    }

    pub mod use_cases {
        pub mod auth {
            pub mod sign_in_use_case;
            pub mod sign_up_use_case;
        }
    }

    pub mod dtos {
        pub mod auth {
            pub mod sign_in_dto;
            pub mod sign_up_dto;
        }
    }
}

pub mod infrastructure {
    pub mod repositories {
        pub mod auth {
            pub mod sign_in_repository;
            pub mod sign_up_repository;
        }
    }

    pub mod adapters {
        pub mod bcrypt {
            pub mod bcrypt_adapter;
        }

        pub mod uuid {
            pub mod uuid_adapter;
        }

        pub mod axum {
            pub mod axum_adapter;
        }

        pub mod regex {
            pub mod regex_adapter;
        }
    }

    pub mod gateways {
        pub mod database {
            pub mod database_gateway;
        }
    }

    pub mod boostrap {
        pub mod api {
            pub mod api_boostrap;
        }
    }

    pub mod factories {
        pub mod controller {
            pub mod auth {
                pub mod sign_up_controller_factory;
            }
        }
    }
}

pub mod presentation {
    pub mod dtos {
        pub mod http {
            pub mod http_request_dto;
            pub mod http_response_dto;
        }
    }

    pub mod controllers {
        pub mod auth {
            pub mod sign_up_controller;
            pub mod sign_up_validator;
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
