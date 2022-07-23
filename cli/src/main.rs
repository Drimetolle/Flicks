use flicks_core::discord_client::DiscordClient;
use flicks_core::avatars_providers::avatars_providers::{AvatarProvider, AvatarFileProvider};

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    let token = dotenv!("ACCESS_TOKEN");
    let base_path = dotenv!("BASE_PATH");
    let image = dotenv!("IMAGE_NAME");
    
    let client = DiscordClient::new(token);
    let avatar_provider = AvatarFileProvider::new(base_path.to_string());
    let image = avatar_provider.get(image);
    client.change_user_picture(image.unwrap()).await;
}