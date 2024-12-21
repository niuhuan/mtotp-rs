use std::fmt::{Display, Formatter};
use hmac::digest::InvalidLength;

#[derive(Debug)]
pub enum Error {
    Hmac(InvalidLength),
    Message(ErrorMessage),
    Other(Box<dyn std::error::Error + Sync + Send>),
}

impl Error {
    pub(crate) fn message(content: impl Into<String>) -> Self {
        Self::Message(ErrorMessage {
            content: content.into(),
        })
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_struct("apple_development::Error");
        match self {
            Error::Hmac(err) => {
                builder.field("kind", &"Hmac");
                builder.field("source", err);
            }
            Error::Message(err) => {
                builder.field("kind", &"Message");
                builder.field("source", err);
            }
            Error::Other(err) => {
                builder.field("kind", &"Other");
                builder.field("source", err);
            }
        }
        builder.finish()
    }
}

impl std::error::Error for Error {}

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Default, Debug, Clone)]
pub struct ErrorMessage {
    pub content: String,
}

impl Display for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_struct("apple_development::ErrorMessage");
        builder.field("content", &self.content);
        builder.finish()
    }
}

impl std::error::Error for ErrorMessage {}

impl From<InvalidLength> for Error {
    fn from(value: InvalidLength) -> Self {
        Self::Hmac(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Other(Box::new(value))
    }
}