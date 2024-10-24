use flicks_discord::discord_client::DiscordClient;
use flicks_telegram::telegram_client::TelegramClient;

use file_system_provider::avatar_providers::AvatarFileProvider;
use file_system_provider::file_repository::FileRepository;
use file_system_provider::image_storage::ImageStorage;
use flicks_core::avatar_pipeline::AvatarPipeline;

use std::io::{self, BufRead as _, Write as _};

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = dotenv!("DISCORD_ACCESS_TOKEN").to_string();
    let super_properties_header = dotenv!("DISCORD_SUPER_PROPERTIES").to_string();
    let cookie = dotenv!("DISCORD_SUPER_PROPERTIES").to_string();
    let user_agent = dotenv!("USER_AGENT").to_string();

    let base_path = dotenv!("BASE_PATH").to_string();
    let used_images_path = dotenv!("USED_IMAGES_PATH").to_string();

    let app_id = dotenv!("APP_ID").to_string().parse::<i32>().unwrap();
    let api_hash = dotenv!("API_HASH").to_string();
    let session_file = dotenv!("SESSION_FILE").to_string();
    let phone = dotenv!("PHONE").to_string();
    let password = dotenv!("PASSWORD").to_string();

    let repository = FileRepository::new();
    let storage = ImageStorage::new(base_path, used_images_path, repository);

    let discord_client = DiscordClient::new(token, super_properties_header, cookie, user_agent);
    let telegram_client = TelegramClient::create(app_id, api_hash, session_file).await?;
    telegram_client
        .auth(phone, password, get_verification_code)
        .await?;

    let avatar_provider = AvatarFileProvider::new(storage);
    let mut pipeline = AvatarPipeline::new(&avatar_provider);

    pipeline
        .add(discord_client)
        .add(telegram_client)
        .run()
        .await?;

    println!("Avatar updated successfully");

    Ok(())
}

fn get_verification_code() -> String {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = stdout.write_all("Inter telegram code: ".as_bytes());
    let _ = stdout.flush();

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    let _ = stdin.read_line(&mut line);

    line
}
