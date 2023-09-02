use grammers_client::{Client, Config, InitParams, SignInError};
use grammers_session::Session;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct TelegramClient {
    client: Client,
    api_id: i32,
    api_hash: String,
    session_file: String,
}

impl TelegramClient {
    pub async fn create(api_id: i32, api_hash: String, session_file: String) -> Result<Self> {
        let config = Config {
            session: Session::load_file_or_create(&session_file)?,
            api_id,
            api_hash: api_hash.clone(),
            params: InitParams {
                catch_up: true,
                ..Default::default()
            },
        };

        let client = Client::connect(config).await?;

        Ok(TelegramClient {
            client,
            api_id,
            api_hash,
            session_file,
        })
    }

    pub async fn auth(
        &self,
        phone: String,
        password: String,
        varification_code_callback: fn() -> String,
    ) -> Result<()> {
        if !self.client.is_authorized().await? {
            let token = self
                .client
                .request_login_code(&phone, self.api_id, &self.api_hash)
                .await?;
            let code = varification_code_callback();
            let signed_in = self.client.sign_in(&token, &code).await;

            match signed_in {
                Err(SignInError::PasswordRequired(password_token)) => {
                    self.client
                        .check_password(password_token, password.trim())
                        .await?;
                }
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            };

            match self.client.session().save_to_file(&self.session_file) {
                Ok(_) => {}
                Err(e) => {
                    println!(
                        "NOTE: failed to save the session, will sign out when done: {}",
                        e
                    );
                }
            }
        }

        Ok(())
    }
}
