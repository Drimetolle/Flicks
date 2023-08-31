pub struct Image {
    pub name: String,
    /// A base64-encoded string as the avatar content.
    pub data: String,
}

impl Image {
    pub fn new(name: String, data: String) -> Self {
        Self {
            name,
            data
        }
    }
}