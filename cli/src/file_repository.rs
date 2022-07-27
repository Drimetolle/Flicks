use std::fs;
use crate::PathBuf;
use std::fs::File;
use std::io::Read;
use std::ffi::OsStr;

pub struct FileRepository {
    base_path: String
}

impl FileRepository {
    pub fn new(base_path: String) -> Self {
        Self {
            base_path
        }
    }

    pub fn get_list_of_images(&self) -> Vec<PathBuf> {
        let paths: Vec<PathBuf> = fs::read_dir(&self.base_path)
        .unwrap_or_else(|error| {
            panic!("Directory posibble undefined: {:?}", error);
        })
        .filter(|path| {
            let path = path.as_ref();
    
            if path.is_ok() {
                let is_image = path
                    .unwrap()
                    .path()
                    .extension()
                    .map_or(false, |extension| extension.eq("jpg") || extension.eq("png"));
    
                return is_image;
            }
    
            false
        })
        .map(|dir_entity| dir_entity.unwrap().path())
        .collect();
    
        return paths;
    }

    pub fn read_image(&self, path: &PathBuf) -> Result<String, std::io::Error> {
        let mut v = Vec::default();
        let mut f = File::open(path)?;
    
        drop(f.read_to_end(&mut v));
    
        let b64 = base64::encode(&v);
        let ext = if path.extension() == Some(OsStr::new("png")) { "png" } else { "jpg" };
    
        Ok(format!("data:image/{};base64,{}", ext, b64))
    }
}