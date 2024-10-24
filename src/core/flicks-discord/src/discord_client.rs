use async_trait::async_trait;
use std::error::Error;
use std::{ffi::OsStr, path::Path};
use flicks_core::avatar_pipeline::AvatarChangeStage;
use flicks_core::image::Image;
use flicks_core::errors::ChangeAvatarError;
use reqwest::{Client};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, COOKIE, USER_AGENT};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::Serialize;
use http::StatusCode;

pub struct DiscordClient {
    client: Client,
}

#[derive(Serialize)]
pub struct AvatarChangeRequest {
    avatar: String,
}

impl DiscordClient {
    pub fn new(token: String, super_properties_header: String, cookie: String, user_agent: String) -> Self {
        let mut headers = HeaderMap::new();

        let header_value = HeaderValue::from_str(token.as_str()).expect("Invalid Authorization header");
        headers.insert(AUTHORIZATION, header_value);

        let header_value = HeaderValue::from_str(super_properties_header.as_str()).expect("Invalid X-Super-Properties header");
        headers.insert("X-Super-Properties", header_value);

        let header_value = HeaderValue::from_str(user_agent.as_str()).expect("Invalid User-Agent header");
        headers.insert(USER_AGENT, header_value);

        let header_value = HeaderValue::from_str(cookie.as_str()).expect("Invalid Cookie header");
        headers.insert(COOKIE, header_value);

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Cannot build http client");

        Self { client }
    }
}

#[async_trait]
impl AvatarChangeStage for DiscordClient {
    async fn change_avatar(&self, image: &Image) -> Result<(), Box<dyn Error>> {
        let b64 = STANDARD.encode(&image.bytes);
        let path = Path::new(&image.name);

        let extension = if path.extension() == Some(OsStr::new("png")) {
            "png"
        } else {
            "jpg"
        };

        let base64_image = format!("data:image/{};base64,{}", extension, b64);
        let request = AvatarChangeRequest {
            avatar: base64_image
        };

        let result = self.client.patch("https://discord.com/api/v9/users/@me")
            .json(&request)
            .send()
            .await?;

        match result.status() {
            StatusCode::OK => Ok(()),
            _ =>  Err(Box::new(ChangeAvatarError::new(result.text().await?)))
        }
    }
}