pub struct Image {
    pub name: String,
    pub bytes: Vec<u8>,
}

impl Image {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        Self { name, bytes: data }
    }
}
