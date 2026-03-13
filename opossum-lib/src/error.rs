use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpossumError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("ZIP error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    #[error("Parse error: {0}")]
    ParseError(String),
}
