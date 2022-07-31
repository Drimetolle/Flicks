use super::image::Image;

pub trait TakeImageCommand {
    fn take(&self) -> Result<Image, Box<dyn std::error::Error>>;
    fn rollback(&self) -> Image;
}