use flicks_core::command::TakeImageCommand;
use flicks_core::image::Image;
use super::FileRepository;
use std::path::PathBuf;
use std::fs;

fn create_if_not_exist(path: &PathBuf) -> std::io::Result<()> {
    if !path.exists() {
        fs::create_dir(path)?
    }

    Ok(())
}

pub struct ImageStorage {
    base_path: PathBuf,
    used_images_path: PathBuf,
    file_repository: Box<FileRepository>
}

impl ImageStorage {
    pub fn new(base_path: String, used_images_path: String, file_repository: FileRepository) -> Self {
        let base_path = PathBuf::from(base_path);
        let used_images_path = PathBuf::from(used_images_path);

        match create_if_not_exist(&base_path) {
            Err(err) => panic!("{}", err),
            _ => ()
        }

        match create_if_not_exist(&used_images_path) {
            Err(err) => panic!("{}", err),
            _ => ()
        }

        Self {
            base_path,
            used_images_path,
            file_repository: Box::new(file_repository)
        }
    }

    pub fn get_image(&self, path: PathBuf) -> TakeImage {
        TakeImage {
            image_path: path,
            used_images_path: &self.used_images_path,
            file_repository: &self.file_repository
        }
    }

    pub fn get_list_of_images(&self) -> Vec<PathBuf> {
        self.file_repository.get_list_of_images(&self.base_path)
    }
}

pub struct TakeImage<'a> {
    image_path: PathBuf,
    used_images_path: &'a PathBuf,
    file_repository: &'a FileRepository
}

impl<'a> TakeImageCommand for TakeImage<'a> {
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

impl<'a> TakeImage<'a> {
    fn get_file_name(&self) -> &str {
        let filename = Some(&self.image_path)
            .and_then(|name| name.file_name())
            .and_then(|name| name.to_str())
            .unwrap();
    
        return filename;
    }
}