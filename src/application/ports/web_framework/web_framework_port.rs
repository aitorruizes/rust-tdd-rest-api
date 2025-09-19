use std::pin::Pin;

pub type ServeFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + 'a>>;

pub trait WebFrameworkPort {
    fn serve(&self) -> ServeFuture<'_>;
}
