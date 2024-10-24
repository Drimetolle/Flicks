use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ChangeAvatarError {
    details: String,
}

impl ChangeAvatarError {
    pub fn new(msg: String) -> ChangeAvatarError {
        ChangeAvatarError { details: msg }
    }
}

impl Error for ChangeAvatarError {}

impl fmt::Display for ChangeAvatarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "change avatar error")
    }
}