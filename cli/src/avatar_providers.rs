use std::path::Path;
use flicks_core::image::Image;
use std::fs::File;
use std::ffi::OsStr;
use std::io::Read;

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

        let avatar = self.read_image(path.as_ref());

        match avatar {
            Ok(base64_image) => Some(Image::new(name, base64_image)),
            Err(_) => None
        };

        None
    }
}

impl AvatarFileProvider {
    fn read_image(&self, path: &Path) -> Result<String, std::io::Error> {
        let mut v = Vec::default();
        let mut f = File::open(path).unwrap();
    
        drop(f.read_to_end(&mut v));
    
        let b64 = base64::encode(&v);
        let ext = if path.extension() == Some(OsStr::new("png")) { "png" } else { "jpg" };
    
        Ok(format!("data:image/{};base64,{}", ext, b64))
    }
}