use serenity::http::{Http, HttpBuilder};

use crate::avatars_providers::image::Image;

pub struct DiscordClient {
    client: Http,
}

impl DiscordClient {
    pub fn new(token: impl AsRef<str>) -> Self {
        let mut client =  HttpBuilder::new("")
            .ratelimiter_disabled(true)
            .build();
        client.token = token.as_ref().to_string();

        Self {
            client
        }
    }

    pub async fn change_user_picture(&self, image: Image) -> Result<(), serenity::Error> {
        let user = self.client.get_current_user().await;

        match user {
            Ok(mut user) => {
                let operation_result = user.edit(&self.client, |p| p.avatar(Some(&image.data))).await;

                match operation_result {
                    Err(err) => Err(err),
                    _ => Ok(())
                }
            },
            Err(err) => Err(err)
        }
    }
}