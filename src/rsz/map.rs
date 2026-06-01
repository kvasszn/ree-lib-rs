use std::collections::HashMap;
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer};

use crate::util::*;

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct RszMap {
    #[serde(deserialize_with = "parse_hex_map")]
    pub types: HashMap<u32, TypeInfo>
}

impl RszMap {
    pub fn get_type(&self, name: &str) -> Option<&TypeInfo> {
        self.types.get(&murmur3(name))
    }

    pub fn get_by_hash(&self, hash: u32) -> Option<&TypeInfo> {
        self.types.get(&hash)
    }

    pub fn get_type_mut(&mut self, name: &str) -> Option<&mut TypeInfo> {
        let hash = murmur3(name);
        self.types.get_mut(&hash)
    }

    pub fn get_by_hash_mut(&mut self, hash: u32) -> Option<&mut TypeInfo> {
        self.types.get_mut(&hash)
    }

    pub fn contains_type(&self, name: &str) -> bool {
        self.types.contains_key(&murmur3(name))
    }

    pub fn get_field_path(&self, start_type: &str, path: &str) -> Option<&FieldInfo> {
        let mut current_type = self.get_type(start_type)?;
        let parts: Vec<&str> = path.split('.').collect();
        for (i, part) in parts.iter().enumerate() {
            let field = current_type.get_field(part)?;
            if i == parts.len() - 1 {
                return Some(field);
            }
            current_type = self.get_type(&field.original_type)?;
        }
        None
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct TypeInfo {
    #[serde(deserialize_with = "parse_address_u32")]
    pub crc: u32,
    pub name: String, 
    #[serde(deserialize_with = "parse_array_to_map")]
    pub fields: IndexMap<u32, FieldInfo>
}

impl TypeInfo {
    pub fn get_field(&self, name: &str) -> Option<&FieldInfo> {
        self.fields.get(&murmur3(name))
    }

    pub fn get_field_idx(&self, name: &str) -> Option<usize> {
        self.fields.get_index_of(&murmur3(name))
    }

    pub fn get_field_mut(&mut self, name: &str) -> Option<&mut FieldInfo> {
        let hash = murmur3(name);
        self.fields.get_mut(&hash)
    }

    pub fn get_by_index(&self, index: usize) -> Option<&FieldInfo> {
        self.fields.get_index(index).map(|(_hash, field)| field)
    }

    pub fn get_by_index_mut(&mut self, index: usize) -> Option<&mut FieldInfo> {
        self.fields.get_index_mut(index).map(|(_hash, field)| field)
    }

}

#[derive(Debug, Clone, Deserialize)]
pub struct FieldInfo {
    pub align: u16,
    pub array: bool,
    pub name: String,
    pub native: bool,
    pub original_type: String,
    pub size: u16,
    pub r#type: String,
}

impl FieldInfo {
    pub fn hash(&self) -> u32 {
        murmur3(&self.name)
    }

    pub fn rename(&mut self, new_name: impl Into<String>) {
        let name_str = new_name.into();
        self.name = name_str;
    }
}

fn parse_array_to_map<'de, D>(deserializer: D) -> Result<IndexMap<u32, FieldInfo>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec: Vec<FieldInfo> = Vec::deserialize(deserializer)?;
    let mut map = IndexMap::with_capacity(vec.len());
    for item in vec {
        map.insert(murmur3(&item.name), item);
    }
    Ok(map)
}

#[macro_export]
macro_rules! rsz_path {
    ($map:expr, $current_type:expr, $field:ident) => {
        $current_type.get_field(stringify!($field))
    };

    ($map:expr, $current_type:expr, $field:ident . $($rest:ident).+) => {
        $current_type.get_field(stringify!($field))
            .and_then(|f| $map.get_type(&f.original_type))
            .and_then(|t| rsz_path!($map, t, $($rest).+))
    };

    ($map:expr, $start_type_name:literal => $($rest:ident).+) => {
        $map.get_type($start_type_name)
            .and_then(|t| rsz_path!($map, t, $($rest).+))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn test_load_real_rsz_dump() {
        let file_path = "./res/rszmhwilds.json";
        let file = File::open(file_path).expect("");

        let reader = BufReader::new(file);

        //println!("Parsing RSZ dump... This might take a second for large files.");
        let _: RszMap = serde_json::from_reader(reader).expect("Failed to parse JSON");
        //println!("Successfully parsed {} types!", map.types.len());
    }
}
