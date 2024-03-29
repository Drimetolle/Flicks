use crate::image_storage::ImageStorage;
use async_trait::async_trait;
use flicks_core::avatar_pipeline::AvatarProvider;
use flicks_core::command::TakeImageCommand;
use flicks_core::image::Image;
use rand::seq::SliceRandom;

pub struct AvatarFileProvider {
    storage: Box<ImageStorage>,
}

impl AvatarFileProvider {
    pub fn new(storage: ImageStorage) -> Self {
        Self {
            storage: Box::new(storage),
        }
    }
}

#[async_trait]
impl AvatarProvider for AvatarFileProvider {
    async fn get(&self) -> Option<Image> {
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
                let result = avatar.rollback();

                if result.is_err() {
                    panic!(
                        "Something went wrong while rollback operation: {}",
                        result.err().unwrap()
                    )
                }

                None
            }
        };

        result
    }
}
