use crate::assets::bundle::Bundle;
use crate::enums::EnumMap;
use crate::language::{Language};
use crate::{rsz::{RszMap, Value}};
use crate::types::Guid;

pub struct EngineContext<'a> {
    pub language: Language,
    pub rsz_map: &'a RszMap, // TODO: possibly handle il2cpp as a source of type info through
                             // generics
    pub assets: &'a Bundle,
    pub enums: &'a EnumMap,
}

impl<'a> EngineContext<'a> {
    pub fn new(language: Language, rsz_map: &'a RszMap, assets: &'a Bundle, enums: &'a EnumMap) -> Self {
        Self {
            language,
            rsz_map,
            assets,
            enums
        }
    }

    pub fn query_rsz_array(
        &self, 
        rsz_file: &str, 
        array_path: &str, 
        match_field: &str, 
        match_value: &Value
    ) -> Option<&'a Value> {
        let rsz = self.assets.get_rsz(rsz_file).ok()?;
        let array_val = rsz.get(array_path, self.rsz_map)?;
        let array = match array_val {
            Value::Array(arr) => arr,
            _ => return None,
        };

        for item in array {
            if let Value::Object(obj_id) = item {
                let instance = rsz.instances.get(*obj_id as usize)?;
                let type_info = self.rsz_map.get_by_hash(instance.hash)?;
                if let Some(field_idx) = type_info.get_field_idx(match_field)
                    && let Some(field_val) = instance.fields.get(field_idx)
                        && field_val.loose_eq(match_value) {
                            return Some(item);
                }
            }
        }

        None
    }

    pub fn get_msg_entry(&self, msg_file: &str, guid: &Guid) -> Option<String> {
        let msg = self.assets.get_msg(msg_file).ok()?;
        msg.get_entry(guid, self.language).map(|s| s.to_string())
    }
}
