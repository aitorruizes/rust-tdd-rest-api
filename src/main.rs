pub mod domain {
    pub mod entities {
        pub mod user {
            pub mod user_entity;
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
    }
}

fn main() {
    println!("Hello, world!");
}
