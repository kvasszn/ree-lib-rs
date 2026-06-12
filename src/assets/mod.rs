pub mod user;
pub mod font;
pub mod error;
pub mod msg;
pub mod bundle;

use std::io::Cursor;

pub use user::*;
pub use msg::*;
pub use font::*;

use error::{FileReadError, Result};

use crate::{rsz::RszMap};

pub enum GameAsset {
    User(UserFile),
    Msg(MsgFile),
    Oft(Oft),
    Prefab
}

#[derive(Debug)]
pub enum FileType {
    User,
    Msg,
    Oft,
    Prefab,
    Mesh,
    Texture,
    Pog,
    Unk
}

impl FileType {
    pub fn get_file_type_from_path(path: &str) -> (Self, Option<u32>) {
        let mut ext = path.split('.').rev().peekable();
        let version = ext.peek().and_then(|e| e.parse::<u32>().ok());
        for s in ext {
            let t = match s {
                "user" => Self::User,
                "msg" => Self::Msg,
                "pdb" => Self::Prefab,
                "mesh" => Self::Mesh,
                "tex" => Self::Texture,
                "oft" => Self::Oft,
                _ => continue
            };
            return (t, version);
        }
        (Self::Unk, version)
    }
}

impl GameAsset {
    pub fn load_asset(path: &str, rsz_map: &RszMap) -> Result<Self> {
        use FileType::*;
        let (ty, _vers) = FileType::get_file_type_from_path(path);
        let data = std::fs::read(path)?;
        let res = match ty {
            User => {
                let mut data = Cursor::new(data);
                Self::User(UserFile::read(&mut data, rsz_map)?)
            },
            Msg => Self::Msg(MsgFile::read(&data)?),
            _ => Err(FileReadError::UnknownFileType(path.to_string()))?
        };
        Ok(res)
    }
}
