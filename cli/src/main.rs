use std::fs;
use std::path::PathBuf;
use flicks_core::discord_client::DiscordClient;
mod avatar_providers;
use crate::avatar_providers::{AvatarProvider, AvatarFileProvider};

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    let token = dotenv!("ACCESS_TOKEN");
    let base_path = dotenv!("BASE_PATH");
    let paths = get_images_files(base_path.to_string());
    let path = paths.first();

    if path.is_none() {
        panic!("Not presented any image");
    }

    let client = DiscordClient::new(token);
    let avatar_provider = AvatarFileProvider::new(base_path.to_string());

    let image = avatar_provider.get(path.unwrap().to_str().unwrap());

    let result = client.change_user_picture(image.unwrap()).await;

    match result {
        Err(err) => panic!("{:?}", err),
        _ => println!("Avatar updated sucsesfully")
    }
}

fn get_images_files(base_path: String) -> Vec<PathBuf> {
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
                .and_then(|extension| extension.to_str())
                .map_or(false, |extension| extension.eq("jpg") || extension.eq("png"));

            return is_image;
        }

        false
    })
    .map(|dir_entity| dir_entity.unwrap().path())
    .collect();

    return paths;
}