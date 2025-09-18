use crate::{
    application::ports::{
        environment::environment_port::EnvironmentPort, logger::logger_port::LoggerPort,
        logger_subscriber::logger_subsriber_port::LoggerSubscriberPort,
    },
    infrastructure::adapters::{
        dotenvy::dotenvy_adapter::DotenvyAdapter, tracing::tracing_adapter::TracingAdapter,
        tracing_subscriber::tracing_subscriber_adapter::TracingSubscriberAdapter,
    },
};

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
    }
}

pub mod presentation {
    pub mod dtos {
        pub mod user {
            pub mod create_user_dto;
        }
    }
}

fn main() {
    let tracing_subscriber_adapter: TracingSubscriberAdapter = TracingSubscriberAdapter;

    tracing_subscriber_adapter.initialize();

    let tracing_adapter: TracingAdapter = TracingAdapter;
    let dotenvy_adapter: DotenvyAdapter = DotenvyAdapter::new();

    match dotenvy_adapter.load_environment_file() {
        Ok(_) => tracing_adapter.log_info("Environment file successfully loaded."),
        Err(err) => tracing_adapter.log_error(&err.to_string()),
    };
}
