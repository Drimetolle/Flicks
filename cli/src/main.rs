use flicks_core::discord_client::DiscordClient;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    let token = dotenv!("ACCESS_TOKEN");

    let client = DiscordClient::new(token);
}