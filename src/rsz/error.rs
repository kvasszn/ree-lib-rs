use thiserror::Error;

use crate::rsz::{FieldInfo, TypeInfo};


#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("Hash {0:x} not found in type map")]
    HashNotFound(u32),
    #[error("Unhandled Field Type {} in {}.{}", .0.r#type, .1.name, .0.name)]
    UnhandledFieldType(FieldInfo, TypeInfo),
    #[error("Unknown or unsupported type {0}")]
    UnknownType(String),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum RszError {
    #[error("Expected magic {0:?}, found {1:?}")]
    InvalidMagic([u8; 4], [u8; 4]),
    #[error("Expected version {0}, found {1}")]
    InvalidVersion(u32, u32),
    #[error("Object Index {0} for hash {1:08x} out of bounds for len {2}")]
    ObjectIndexOutOfBounds(u32, u32, usize),
    #[error("Extern Hash {0} not equal to Type Descriptor hash {1:08x} at index {2}")]
    ExternHashNotEqual(u32, u32, usize),
    #[error("Misc {0}")]
    Misc(String),
    #[error("IO error {0}")]
    IO(#[from] std::io::Error),
    #[error("Deserialize error: {0}")]
    DeserializeError(#[from] DeserializeError),
}

impl From<String> for RszError {
    fn from(str: String) -> Self {
        Self::Misc(str)
    }
}

pub type Result<T> = std::result::Result<T, RszError>;
