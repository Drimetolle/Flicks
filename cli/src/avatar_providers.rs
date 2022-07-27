use flicks_core::image::Image;
use crate::FileRepository;

pub trait AvatarProvider {
    fn get(&self) -> Option<Image>;
}

pub struct AvatarFileProvider {
    repository: Box<FileRepository>,
}

impl AvatarFileProvider{
    pub fn new(repository: FileRepository) -> Self {
        Self { repository: Box::new(repository) }
    }
}

impl AvatarProvider for AvatarFileProvider {
    fn get(&self) -> Option<Image> {
        let paths = self.repository.get_list_of_images();
        let path = paths.first();

        if path.is_none() {
            return None;
        }

        let path = path.unwrap();
        let avatar = self.repository.read_image(&path);

        let result = match avatar {
            Ok(base64_image) => Some(Image::new(path.file_name().unwrap().to_str().unwrap().to_string(), base64_image)),
            Err(_) => None
        };

        result
    }
}
