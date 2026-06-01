pub mod user;
pub mod error;

pub use user::*;

pub enum GameAsset {
    User(UserFile),
    Msg,
    Prefab
}
