use std::path::Path;

use anyhow::Result;
use ree_lib::{enums::{EnumMap, load_enum_map}, rsz::{RszMap}};

pub fn main() -> Result<()> {
    let rsz_map = std::fs::read_to_string("/home/nikola/programming/mhtame/assets/mhrise/rszmhrise.json")?;
    let rsz_map: RszMap = serde_json::from_str(&rsz_map)?;
    let enum_map: EnumMap = load_enum_map(Path::new("/home/nikola/programming/ree-save-editor/assets/raw_enums/enumsmhrise.hpp"))?;
    println!("{:#?}", enum_map);
    Ok(())
}
