use async_trait::async_trait;
use serenity::http::{Http, HttpBuilder};
use std::error::Error;
use std::{ffi::OsStr, path::Path};

use flicks_core::avatar_pipeline::AvatarChangeStage;
use flicks_core::image::Image;
use reqwest::{Client};
use reqwest::header::{HeaderMap, HeaderValue};

pub struct DiscordClient {
    client: Http,
}

impl DiscordClient {
    pub fn new(token: String, super_properties_header: String) -> Self {
        let mut headers = HeaderMap::new();

        let header_value = HeaderValue::from_str(token.as_str()).expect("Invalid Authorization header");
        headers.insert("Authorization", header_value);

        let header_value = HeaderValue::from_str(super_properties_header.as_str()).expect("Invalid X-Super-Properties header");
        headers.insert("X-Super-Properties", header_value);

        let client = Client::builder()
            .default_headers(headers)
            .use_rustls_tls()
            .build()
            .expect("Cannot build http client");

        let mut client = HttpBuilder::new("").client(client).ratelimiter_disabled(true).build();
        client.token = token;

        Self { client }
    }
}

#[async_trait]
impl AvatarChangeStage for DiscordClient {
    async fn change_avatar(&self, image: &Image) -> Result<(), Box<dyn Error>> {
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
                    Err(err) => Err(Box::new(err)),
                    _ => Ok(()),
                }
            }
            Err(err) => Err(Box::new(err)),
        }
    }
}
