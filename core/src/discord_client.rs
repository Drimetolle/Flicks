use serenity::http::{Http, HttpBuilder};

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
}