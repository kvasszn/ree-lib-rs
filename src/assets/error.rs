use thiserror::Error;

use crate::rsz::error::RszError;

#[derive(Error, Debug)]
pub enum FileReadError {
    #[error("Expected magic {0:?}, found {1:?}")]
    InvalidMagic([u8; 4], [u8; 4]),
    #[error("Expected version {0}, found {1}")]
    InvalidVersion(u32, u32),
    #[error("Failed to parse Rsz {0}")]
    RszError(#[from] RszError),
    #[error("IO error {0}")]
    IO(#[from] std::io::Error),
    #[error("Unknown File Type: {0}")]
    UnknownFileType(String),
}

pub type Result<T> = std::result::Result<T, FileReadError>;
