use serenity::http::{Http, HttpBuilder};
use std::{ffi::OsStr, path::Path};

use flicks_core::image::Image;

pub struct DiscordClient {
    client: Http,
}

impl DiscordClient {
    pub fn new(token: impl AsRef<str>) -> Self {
        let mut client = HttpBuilder::new("").ratelimiter_disabled(true).build();
        client.token = token.as_ref().to_string();

        Self { client }
    }

    pub async fn change_user_picture(&self, image: &Image) -> Result<(), serenity::Error> {
        let user = self.client.get_current_user().await;

        let b64 = base64::encode(&image.bytes);
        let path = Path::new(&image.name);

        let extension = if path.extension() == Some(OsStr::new("png")) {
            "png"
        } else {
            "jpg"
        };

        let base64_image = format!("data:image/{};base64,{}", extension, b64);

        match user {
            Ok(mut user) => {
                let operation_result = user
                    .edit(&self.client, |p| p.avatar(Some(&base64_image)))
                    .await;

                match operation_result {
                    Err(err) => Err(err),
                    _ => Ok(()),
                }
            }
            Err(err) => Err(err),
        }
    }
}
