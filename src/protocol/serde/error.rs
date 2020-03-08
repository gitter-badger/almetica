use std;
use std::fmt::Display;

use serde::{de, ser};
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error("DeserializeAnyNotSupported. Pos: {0}")]
    DeserializeAnyNotSupported(usize),

    #[error("InvalidBoolEncoding. Val: {0} Pos: {1}")]
    InvalidBoolEncoding(u8, usize),

    #[error("InvalidCharEncoding. Pos: {0}")]
    InvalidCharEncoding(usize),

    #[error("StringNotNullTerminated. Pos: {0}")]
    StringNotNullTerminated(usize),

    #[error("InvalidTagEncoding. Tag: {0} Pos: {1}")]
    InvalidTagEncoding(u8, usize),

    #[error("DeserializeMapNotSupported. Pos: {0}")]
    DeserializeMapNotSupported(usize),

    #[error("DeserializeIdentifierNotSupported. Pos: {0}")]
    DeserializeIdentifierNotSupported(usize),

    #[error("DeserializeIgnoredAnyNotSupported. Pos: {0}")]
    DeserializeIgnoredAnyNotSupported(usize),

    #[error("BytesTooBig. Pos: {0}")]
    BytesTooBig(usize),

    #[error("serde error: {0}")]
    Serde(#[from] serde_yaml::Error),
}

impl de::Error for Error {
    fn custom<T: Display>(desc: T) -> Error {
        Error::Custom(desc.to_string()).into()
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string()).into()
    }
}
