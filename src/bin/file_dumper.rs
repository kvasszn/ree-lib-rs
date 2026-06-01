use std::{io::Cursor, path::Path};

use anyhow::Result;
use ree_lib::{assets::UserFile, enums::{EnumMap, load_enum_map}, rsz::{RszContext, RszMap}};
use serde::Serialize;

pub fn main() -> Result<()> {
    /*env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();*/

    let rsz_map = std::fs::read_to_string("./res/rszmhwilds.json")?;
    let rsz_map: RszMap = serde_json::from_str(&rsz_map)?;
    let enum_map: EnumMap = load_enum_map(Path::new("./res/enumsmhwilds.hpp"))?;
    //println!("{:#?}", enum_map);
    let data = std::fs::read("../wilds_files/natives/STM/GameDesign/Common/Enemy/EM0001_00_0.user.3")?;
    let mut reader = Cursor::new(data);
    let user_file = UserFile::read(&mut reader, &rsz_map)?;
    let rsz_ctx = RszContext::new(&user_file.rsz, &rsz_map, &enum_map);
    let rsz_str = serde_json::to_string_pretty(&rsz_ctx)?;
    println!("{}", rsz_str);
    Ok(())
}
