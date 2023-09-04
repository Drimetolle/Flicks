use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct FileRepository;

impl FileRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_list_of_images(&self, path: &PathBuf) -> Vec<PathBuf> {
        let paths: Vec<PathBuf> = fs::read_dir(path)
            .unwrap_or_else(|error| {
                panic!("Directory posibble undefined: {:?}", error);
            })
            .filter(|path| {
                let path = path.as_ref();

                if path.is_ok() {
                    let is_image = path.unwrap().path().extension().map_or(false, |extension| {
                        extension.eq("jpg") || extension.eq("png")
                    });

                    return is_image;
                }

                false
            })
            .map(|dir_entity| dir_entity.unwrap().path())
            .collect();

        return paths;
    }

    pub fn read_image(&self, path: &PathBuf) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes = Vec::default();
        let mut file: File = File::open(path)?;

        drop(file.read_to_end(&mut bytes));

        Ok(bytes)
    }

    pub fn move_image(&self, path_from: &PathBuf, path_to: &PathBuf) -> Result<(), std::io::Error> {
        fs::rename(path_from, path_to)
    }
}
