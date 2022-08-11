use flicks_core::command::TakeImageCommand;
use flicks_core::image::Image;
use super::FileRepository;
use std::path::PathBuf;

pub struct ImageStorage {
    base_path: String,
    used_images_path: PathBuf,
    file_repository: Box<FileRepository>
}

impl ImageStorage {
    pub fn new(base_path: String, used_images_path: String, file_repository: FileRepository) -> Self {
        Self {
            base_path,
            used_images_path: PathBuf::from(used_images_path),
            file_repository: Box::new(file_repository)
        }
    }

    pub fn get_image(&self, path: PathBuf) -> TakeImage {
        TakeImage {
            image_path: path,
            used_images_path: self.used_images_path.clone(),
            file_repository: Box::new(*self.file_repository.clone())
        }
    }

    pub fn get_list_of_images(&self) -> Vec<PathBuf> {
        self.file_repository.get_list_of_images(&self.base_path)
    }
}

pub struct TakeImage {
    image_path: PathBuf,
    used_images_path: PathBuf,
    file_repository: Box<FileRepository>
}

impl TakeImageCommand for TakeImage {
    fn take(&self) -> Result<Image, Box<(dyn std::error::Error + 'static)>> { 
        let image_path = &self.image_path;
        let image = self.file_repository.read_image(image_path)?;
        let filename = self.get_file_name();

        self.file_repository.move_image(image_path, &self.used_images_path.join(filename))?;

        Ok(Image { name: filename.to_string(), data: image })
    }

    fn rollback(&self) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        self.file_repository.move_image(&self.used_images_path.join(self.get_file_name()), &self.image_path)?;
        Ok(())
    }
}

impl TakeImage {
    fn get_file_name(&self) -> &str {
        let filename = Some(&self.image_path)
            .and_then(|name| name.file_name())
            .and_then(|name| name.to_str())
            .unwrap();
    
        return filename;
    }
}