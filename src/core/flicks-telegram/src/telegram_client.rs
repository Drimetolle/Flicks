use grammers_client::{types::media::Uploaded, Client, Config, InitParams, SignInError};
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

    // todo use in the future
    async fn upload(&self, image: Image) -> ResultInner<Uploaded> {
        let mut stream = std::io::Cursor::new(&image.bytes);

        let size = image.bytes.len();
        let uploaded_file = self
            .client
            .upload_stream(&mut stream, size, image.name)
            .await?;

        Ok(uploaded_file)
    }

    /// Copy&paste from upload_stream
    async fn upload_file(&self, bytes: &Vec<u8>) -> Result<(i64, String), tokio::io::Error> {
        const MAX_CHUNK_SIZE: usize = 512 * 1024;

        let file_id = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("system time is before epoch")
            .as_nanos() as i64;

        let parts = bytes.chunks(MAX_CHUNK_SIZE);
        let mut md5 = md5::Context::new();

        for (part, chunk) in parts.enumerate() {
            md5.consume(&bytes);
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

        Ok((file_id, md5_checksum))
    }
}

#[async_trait]
impl AvatarChangeStage for TelegramClient {
    async fn change_avatar(&self, image: &Image) -> Result<(), Box<dyn Error>> {
        let (file_id, md5_checksum) = self.upload_file(&image.bytes).await?;

        let photo = InputFile {
            id: file_id,
            parts: 1,
            name: image.name.clone(),
            md5_checksum,
        };
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
