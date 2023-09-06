use flicks_discord::discord_client::DiscordClient;
use flicks_telegram::telegram_client::TelegramClient;

use file_system_provider::avatar_providers::{AvatarFileProvider, AvatarProvider};
use file_system_provider::file_repository::FileRepository;
use file_system_provider::image_storage::ImageStorage;

use std::io::{self, BufRead as _, Write as _};

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = dotenv!("ACCESS_TOKEN");
    let base_path = dotenv!("BASE_PATH").to_string();
    let used_images_path = dotenv!("USED_IMAGES_PATH").to_string();
    let app_id = dotenv!("APP_ID").to_string().parse::<i32>().unwrap();
    let api_hash = dotenv!("API_HASH").to_string();
    let session_file = dotenv!("SESSION_FILE").to_string();
    let phone = dotenv!("PHONE").to_string();
    let password = dotenv!("PASSWORD").to_string();

    let repository = FileRepository::new();
    let storage = ImageStorage::new(base_path, used_images_path, repository);

    let discord_client = DiscordClient::new(token);
    let telegram_client = TelegramClient::create(app_id, api_hash, session_file).await?;
    telegram_client
        .auth(phone, password, get_varification_code)
        .await?;

    let avatar_provider = AvatarFileProvider::new(storage);

    let image = avatar_provider.get().unwrap();

    discord_client.change_user_picture(&image).await?;
    telegram_client.change_user_picture(&image).await?;

    println!("Avatar updated successfully");

    Ok(())
}

fn get_varification_code() -> String {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all("Inter telegram code: ".as_bytes());
    stdout.flush();

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    stdin.read_line(&mut line);

    line
}
