use std::path::PathBuf;
use flicks_core::discord_client::DiscordClient;

mod avatar_providers;
use crate::avatar_providers::{AvatarProvider, AvatarFileProvider};

mod file_repository;
use crate::file_repository::FileRepository;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    let token = dotenv!("ACCESS_TOKEN");
    let base_path = dotenv!("BASE_PATH");

    let repository = FileRepository {};
    let paths = repository.get_list_of_images(base_path.to_string());
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