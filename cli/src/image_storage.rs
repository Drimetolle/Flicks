use flicks_core::command::TakeImageCommand;
use flicks_core::image::Image;
use super::FileRepository;
use std::path::PathBuf;

pub struct ImageStorage {
    base_path: String,
    file_repository: Box<FileRepository>
}

impl ImageStorage {
    pub fn new(base_path: String, file_repository: FileRepository) -> Self {
        Self {
            base_path,
            file_repository: Box::new(file_repository)
        }
    }

    pub fn get_image(&self, path: PathBuf) -> TakeImage {
        TakeImage {
            image_path: path,
            file_repository: Box::new(*self.file_repository.clone())
        }
    }

    pub fn get_list_of_images(&self) -> Vec<PathBuf> {
        self.file_repository.get_list_of_images(&self.base_path)
    }
}

pub struct TakeImage {
    image_path: PathBuf,
    file_repository: Box<FileRepository>
}

impl TakeImageCommand for TakeImage {
    fn take(&self) -> Result<Image, Box<(dyn std::error::Error + 'static)>> { 
        let image_path = &self.image_path;
        let image = self.file_repository.read_image(image_path)?;

        let filename = Some(image_path)
            .as_ref()
            .and_then(|name| name.file_name())
            .and_then(|name| name.to_str())
            .unwrap();

        Ok(Image { name: filename.to_string(), data: image })
    }

    fn rollback(&self) -> Image {
       todo!()
    }
}