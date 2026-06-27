use bincode::{Decode, Encode};
use thiserror::Error;
use std::collections::{HashMap, HashSet};

use crate::{assets::{FileType, GameAsset, MsgFile, UserFile, error::FileReadError}, rsz::{Rsz, RszMap}};


#[derive(Error, Debug)]
pub enum AssetError {
    #[error("Failed to read file {0:?}")]
    FileReadError(#[from] FileReadError),
    #[error("Could not find asset {0} in bundle")]
    AssetNotFound(String),
    #[error("Asset at {1} is not a {0:?}")]
    WrongFileType(FileType, String),
    #[error("{0} does not contain Rsz")]
    NoRsz(String),
}

pub type Result<T> = std::result::Result<T, AssetError>;

// This is a storage container for assets and necessary data for reading from them
#[derive(Default, Encode, Decode)]
pub struct Bundle {
    // path of the asset, the asset
    pub loaded: HashMap<String, GameAsset>,
}

impl Bundle {
    pub fn load_from_paths(&mut self, paths: HashSet<String>, rsz_map: &RszMap) {
        for path in paths {
            if let Err(e) = self.load_asset(&path, rsz_map) {
                log::error!("{e}: Could not load asset {path}.");
            } else {
                log::info!("Loaded asset {path}");
            }
        }
    }

    pub fn load_asset(&mut self, path: &str, rsz_map: &RszMap) -> Result<()> {
        let asset = GameAsset::load_asset(path, rsz_map)?;
        self.loaded.insert(path.to_string(), asset);
        Ok(())
    }

    pub fn get(&self, path: &str) -> Result<&GameAsset> {
        let asset = self.loaded.get(path);
        asset.ok_or(AssetError::AssetNotFound(path.to_string()))
    }

    pub fn get_user(&self, path: &str) -> Result<&UserFile> {
        let asset = self.loaded.get(path);
        let asset = asset.ok_or(AssetError::AssetNotFound(path.to_string()))?;
        if let GameAsset::User(asset) = asset {
            return Ok(asset)
        }
        Err(AssetError::WrongFileType(FileType::User, path.to_string()))
    }

    pub fn get_msg(&self, path: &str) -> Result<&MsgFile> {
        let asset = self.loaded.get(path);
        let asset = asset.ok_or(AssetError::AssetNotFound(path.to_string()))?;
        if let GameAsset::Msg(asset) = asset {
            return Ok(asset)
        }
        Err(AssetError::WrongFileType(FileType::Msg, path.to_string()))
    }

    pub fn get_rsz(&self, path: &str) -> Result<&Rsz> {
        let asset = self.loaded.get(path);
        let asset = asset.ok_or(AssetError::AssetNotFound(path.to_string()))?;
        match asset {
            GameAsset::User(asset) => return Ok(&asset.rsz),
            _ => {}
        }
        Err(AssetError::NoRsz(path.to_string()))
    } 
}
