pub struct DiscordClient {
    token: String,
}

impl DiscordClient {
    pub fn new(token: impl AsRef<str>) -> Self {
        Self {
            token: token.as_ref().to_string()
        }
    }
}