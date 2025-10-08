#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

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

        pub mod repositories {
            pub mod user {
                pub mod create_user_repository_port;
                pub mod get_user_by_email_repository_port;
                pub mod get_user_by_id_repository_port;
            }
        }

        pub mod auth {
            pub mod auth_port;
        }

        pub mod pattern_matching {
            pub mod pattern_matching_port;
        }
    }

    pub mod use_cases {
        pub mod auth {
            pub mod sign_in_use_case;
            pub mod sign_up_use_case;
        }

        pub mod user {
            pub mod get_user_by_id_use_case;
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
        pub mod user {
            pub mod create_user_repository;
            pub mod get_user_by_email_repository;
            pub mod get_user_by_id_repository;
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
            pub mod axum_handler_adapter;
        }

        pub mod regex {
            pub mod regex_adapter;
        }

        pub mod jsonwebtoken {
            pub mod jsonwebtoken_adapter;
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
                pub mod sign_in_controller_factory;
                pub mod sign_up_controller_factory;
            }

            pub mod user {
                pub mod get_user_by_id_controller_factory;
            }
        }
    }

    pub mod mappers {
        pub mod response {
            pub mod user {
                pub mod user_response;
            }
        }
    }

    pub mod models {
        pub mod user {
            pub mod user_model;
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
            pub mod sign_up {
                pub mod sign_up_controller;
                pub mod sign_up_validator;
            }

            pub mod sign_in {
                pub mod sign_in_controller;
                pub mod sign_in_validator;
            }
        }

        pub mod user {
            pub mod get_user_by_id_controller;
        }
    }

    pub mod ports {
        pub mod controller {
            pub mod controller_port;
        }

        pub mod router {
            pub mod router_port;
        }

        pub mod validator {
            pub mod validator_port;
        }
    }

    pub mod routers {
        pub mod auth {
            pub mod auth_router;
        }

        pub mod core {
            pub mod core_router;
        }

        pub mod user {
            pub mod user_router;
        }
    }

    pub mod middlewares {
        pub mod auth {
            pub mod auth_middleware;
        }
    }

    pub mod helpers {
        pub mod http {
            pub mod http_body_helper;
            pub mod http_response_helper;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_boostrap = ApiBootstrap;

    api_boostrap.setup().await
}
