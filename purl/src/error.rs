#[derive(Debug, thiserror::Error)]
pub enum PurlError {
    #[error("Missing required scheme 'pkg:'")]
    MissingScheme,

    #[error("Missing required type component")]
    MissingType,

    #[error("Missing required name component")]
    MissingName,

    #[error("Invalid type: {0}")]
    InvalidType(String),

    #[error("Invalid qualifier key: {0}")]
    InvalidQualifierKey(String),

    #[error("Invalid percent encoding: {0}")]
    InvalidEncoding(String),
}
