use thiserror::Error;

#[derive(Debug, Error)]
pub enum SabError {
    #[error("Io Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Decode Error: {0}")]
    DecodeError(String),

    #[error("Encode Error: {0}")]
    EncodeError(String),
}
