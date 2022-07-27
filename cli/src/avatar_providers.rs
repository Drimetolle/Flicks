use std::path::Path;
use flicks_core::image::Image;

pub trait AvatarProvider {
    fn get(&self, id: impl AsRef<str>) -> Option<Image>;
}

pub struct AvatarFileProvider {
    base_path: String,
}

impl AvatarFileProvider{
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }
}

impl AvatarProvider for AvatarFileProvider {
    fn get(&self, id: impl AsRef<str>) -> Option<Image> {
        let name = id.as_ref().to_string();
        let path = Path::new(&self.base_path).join(&name);

        // TODO implement read file as base64 
        // let avatar = serenity::utils::read_image(path);

        // match avatar {
        //     Ok(base64_image) => Some(Image::new(name, base64_image)),
        //     Err(_) => None
        // }

        None
    }
}