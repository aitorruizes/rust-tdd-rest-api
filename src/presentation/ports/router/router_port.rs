use axum::Router;

pub trait RouterPort {
    fn register_routes(self) -> Router;
}
