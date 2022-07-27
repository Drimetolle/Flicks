use std::fs;
use crate::PathBuf;

pub struct FileRepository;

impl FileRepository {
    pub fn get_list_of_images(&self, base_path: String) -> Vec<PathBuf> {
        let paths: Vec<PathBuf> = fs::read_dir(base_path)
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
}