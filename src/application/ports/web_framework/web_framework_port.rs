use std::pin::Pin;

pub trait WebFrameworkPort {
    fn serve(&self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + '_>>;
}
