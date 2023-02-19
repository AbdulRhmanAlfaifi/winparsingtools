use std::{char::DecodeUtf16Error, str::Utf8Error};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReaderError {
    #[error("an IO error has occurred")]
    IoError(std::io::Error),

    #[error("error while decoding UTF-16: {0}")]
    Utf16Error(DecodeUtf16Error),

    #[error("error while decoding UTF-8: {0}")]
    Utf8Error(Utf8Error)
}

impl From<std::io::Error> for ReaderError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<DecodeUtf16Error> for ReaderError {
    fn from(value: DecodeUtf16Error) -> Self {
        Self::Utf16Error(value)
    }
}

impl From<Utf8Error> for ReaderError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}