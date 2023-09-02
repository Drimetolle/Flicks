use flicks_discord::discord_client::DiscordClient;

use file_system_provider::avatar_providers::{AvatarProvider, AvatarFileProvider};
use file_system_provider::file_repository::FileRepository;
use file_system_provider::image_storage::ImageStorage;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    let token = dotenv!("ACCESS_TOKEN");
    let base_path = dotenv!("BASE_PATH").to_string();
    let used_images_path = dotenv!("USED_IMAGES_PATH").to_string();

    let repository = FileRepository::new();
    let storage = ImageStorage::new(base_path, used_images_path, repository);

    let client = DiscordClient::new(token);
    let avatar_provider = AvatarFileProvider::new(storage);

    let image = avatar_provider.get();

    let result = client.change_user_picture(image.unwrap()).await;

    match result {
        Err(err) => panic!("{:?}", err),
        _ => println!("Avatar updated successfully")
    }
}