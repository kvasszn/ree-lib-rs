use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use indexmap::IndexMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnumValue {
    Signed(i64),
    Unsigned(u64),
}

impl EnumValue {
    pub fn as_u64(&self) -> u64 {
        match self {
            EnumValue::Signed(v) => *v as u64, // Safely bit-casts the negative to u64
            EnumValue::Unsigned(v) => *v,
        }
    }

    pub fn as_i64(&self) -> i64 {
        match self {
            EnumValue::Signed(v) => *v,
            EnumValue::Unsigned(v) => *v as i64,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EnumDefinition{
    pub name_to_value: IndexMap<String, EnumValue>,
    pub value_to_name: IndexMap<EnumValue, String>,
}

impl EnumDefinition {
    pub fn get_value(&self, name: &str) -> Option<EnumValue> {
        self.name_to_value.get(name).copied()
    }

    pub fn get_name(&self, value: EnumValue) -> Option<&String> {
        self.value_to_name.get(&value)
    }

    pub fn get_value_u64(&self, name: &str) -> Option<u64> {
        Some(self.name_to_value.get(name).copied()?.as_u64())
    }

    pub fn get_value_i64(&self, name: &str) -> Option<i64> {
        Some(self.name_to_value.get(name).copied()?.as_i64())
    }

    pub fn get_name_u64(&self, value: u64) -> Option<&String> {
        self.value_to_name.get(&EnumValue::Unsigned(value))
    }

    pub fn get_name_i64(&self, value: i64) -> Option<&String> {
        self.value_to_name.get(&EnumValue::Signed(value))
    }
}

pub type EnumMap = HashMap<String, EnumDefinition>;

fn parse_enum_value(raw: Option<&str>, previous_value: Option<EnumValue>) -> Option<EnumValue> {
    let Some(raw) = raw else {
        return Some(match previous_value {
            Some(EnumValue::Signed(v)) => EnumValue::Signed(v.wrapping_add(1)),
            Some(EnumValue::Unsigned(v)) => EnumValue::Unsigned(v.wrapping_add(1)),
            None => EnumValue::Signed(0),
        });
    };

    let raw = raw.trim();
    let is_explicitly_unsigned = raw.ends_with('u') || raw.ends_with('U');
    let raw = raw.trim_end_matches(['u', 'U']);

    if let Some(hex) = raw.strip_prefix("0x").or_else(|| raw.strip_prefix("0X")) {
        let val = u64::from_str_radix(hex, 16).ok()?;
        if !is_explicitly_unsigned && val <= i32::MAX as u64 {
            return Some(EnumValue::Signed(val as i64));
        } else {
            return Some(EnumValue::Unsigned(val));
        }
    }

    if raw.starts_with('-') {
        let value = raw.parse::<i64>().ok()?;
        return Some(EnumValue::Signed(value));
    }

    if is_explicitly_unsigned {
        let value = raw.parse::<u64>().ok()?;
        Some(EnumValue::Unsigned(value))
    } else {
        let value = raw.parse::<i64>().ok()?;
        Some(EnumValue::Signed(value))
    }
}

pub fn load_enum_map(path: &Path) -> Result<EnumMap> {
    let text = fs::read_to_string(path)?;
    let mut enums = EnumMap::new();

    let mut namespace_stack: Vec<String> = Vec::new();
    let mut current_enum: Option<(String, EnumDefinition)> = None;
    let mut previous_value: Option<EnumValue> = None;

    for line in text.lines() {
        let mut line = line.trim();
        if let Some(idx) = line.find("//") {
            line = line[..idx].trim();
        }
        
        if line.is_empty() {
            continue;
        }

        if line.starts_with('}') {
            if let Some((name, def)) = current_enum.take() {
                let mut full_name = namespace_stack.join(".");
                if !full_name.is_empty() {
                    full_name.push('.');
                }
                full_name.push_str(&name);
                enums.insert(full_name, def);
            } else if !namespace_stack.is_empty() {
                namespace_stack.pop();
            }
            continue;
        }

        if let Some(rest) = line.strip_prefix("namespace ") {
            let ns_name = rest.trim_end_matches('{').trim();
            namespace_stack.push(ns_name.replace("::", "."));
            continue;
        }

        if line.starts_with("enum ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            let name_idx = if parts.get(1) == Some(&"class") { 2 } else { 1 };
            
            if let Some(raw_name) = parts.get(name_idx) {
                let clean_name = raw_name.trim_end_matches(|c| c == '{' || c == ':');
                
                current_enum = Some((clean_name.to_string(), EnumDefinition::default()));
                previous_value = None;
            }
            continue;
        }

        if let Some((_, ref mut def)) = current_enum {
            let line = line.trim_end_matches(',');

            let (name_part, value_part) = if let Some((n, v)) = line.split_once('=') {
                (n.trim(), Some(v.trim()))
            } else {
                (line, None)
            };

            let variant_name = name_part.to_string();

            if let Some(val) = parse_enum_value(value_part, previous_value) {
                previous_value = Some(val);

                def.name_to_value.insert(variant_name.clone(), val);
                def.value_to_name.entry(val).or_insert(variant_name);
            }
        }
    }

    Ok(enums)
}
