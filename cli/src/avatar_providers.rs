use flicks_core::image::Image;
use crate::{image_storage::ImageStorage};
use rand::seq::SliceRandom;
use flicks_core::command::TakeImageCommand;

pub trait AvatarProvider {
    fn get(&self) -> Option<Image>;
}

pub struct AvatarFileProvider {
    storage: Box<ImageStorage>,
}

impl AvatarFileProvider{
    pub fn new(storage: ImageStorage) -> Self {
        Self { storage: Box::new(storage) }
    }
}

impl AvatarProvider for AvatarFileProvider {
    fn get(&self) -> Option<Image> {
        let paths = self.storage.get_list_of_images();
        let path = paths.choose(&mut rand::thread_rng());

        if path.is_none() {
            return None;
        }

        let path = path.unwrap();
        let avatar = self.storage.get_image(path.to_owned());

        let result = match avatar.take() {
            Ok(command) => Some(command),
            Err(_) => {
                avatar.rollback();

                None
            }
        };

        result
    }
}
