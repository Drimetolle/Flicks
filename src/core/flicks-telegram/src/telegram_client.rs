use grammers_client::{Client, Config, InitParams, SignInError};
use grammers_session::Session;
use grammers_tl_types::types::InputFile;
use std::error::Error;

use flicks_core::image::Image;

use async_trait::async_trait;
use flicks_core::avatar_pipeline::AvatarChangeStage;
use std::result::Result;

type ResultInner<T> = Result<T, Box<dyn Error>>;

pub struct TelegramClient {
    client: Client,
    api_id: i32,
    api_hash: String,
    session_file: String,
}

impl TelegramClient {
    pub async fn create(api_id: i32, api_hash: String, session_file: String) -> ResultInner<Self> {
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
        verification_code_callback: fn() -> String,
    ) -> ResultInner<()> {
        if !self.client.is_authorized().await? {
            let token = self
                .client
                .request_login_code(&phone, self.api_id, &self.api_hash)
                .await?;
            let code = verification_code_callback();
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

    async fn upload_file(&self, name: String, bytes: &Vec<u8>) -> Result<InputFile, tokio::io::Error> {
        const MAX_CHUNK_SIZE: usize = 512 * 1024;
        const BIG_FILE_SIZE: usize = 10 * 1024 * 1024;

        if bytes.len() > BIG_FILE_SIZE {
            return Err(tokio::io::Error::new(
                tokio::io::ErrorKind::InvalidData,
                "Big files are not yet supported",
            ));
        }

        let file_id = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("system time is before epoch")
            .as_nanos() as i64;

        let parts = bytes.chunks(MAX_CHUNK_SIZE);
        let mut md5 = md5::Context::new();

        let mut total_parts = 0;
        for (part, chunk) in parts.enumerate() {
            total_parts += 1;
            md5.consume(&chunk);

            let ok = self
                .client
                .invoke(&grammers_tl_types::functions::upload::SaveFilePart {
                    file_id,
                    file_part: part as i32,
                    bytes: chunk.to_owned(),
                })
                .await
                .map_err(|e| tokio::io::Error::new(tokio::io::ErrorKind::Other, e))?;

            if !ok {
                return Err(tokio::io::Error::new(
                    tokio::io::ErrorKind::Other,
                    "server failed to store uploaded data",
                ));
            }
        }

        let md5_checksum = hex::encode(md5.compute());

        Ok(InputFile {
            id: file_id,
            parts: total_parts,
            name,
            md5_checksum,
        })
    }
}

#[async_trait]
impl AvatarChangeStage for TelegramClient {
    async fn change_avatar(&self, image: &Image) -> Result<(), Box<dyn Error>> {
        let photo = self.upload_file(image.name.clone(), &image.bytes).await?;
        let photo = grammers_tl_types::enums::InputFile::File(photo);

        self.client
            .invoke(&grammers_tl_types::functions::photos::UploadProfilePhoto {
                file: Option::from(photo),
                video: None,
                video_start_ts: None,
            })
            .await?;

        Ok(())
    }
}
