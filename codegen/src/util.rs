use std::{path::Path, process::Command};
use ree_lib::il2cpp::{REField, REFieldFlag, REType};

#[derive(Debug, PartialEq)]
pub struct TypeName<'a> {
    pub name: &'a str,
    pub hierarchy: Vec<&'a str>,
    pub generics: Vec<TypeName<'a>>,
}

impl<'a> TypeName<'a> {
    pub fn parse(full_name: &'a str) -> Self {
        let full_name = full_name.trim();

        let (name, generics_str) = match (full_name.find('<'), full_name.rfind('>')) {
            (Some(start), Some(end)) if start < end => {
                (&full_name[..start], &full_name[start + 1..end])
            }
            _ => {
                return Self {
                    name: full_name,
                    hierarchy: full_name.split('.').collect(),
                    generics: Vec::new(),
                }
            }
        };

        Self {
            name,
            hierarchy: name.split('.').collect(),
            generics: Self::split_generics(generics_str)
                .into_iter()
                .map(TypeName::parse)
                .collect(),
        }
    }

    fn split_generics(s: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();
        let mut depth = 0;
        let mut start = 0;

        for (i, c) in s.char_indices() {
            match c {
                '<' => depth += 1,
                '>' => depth -= 1,
                ',' if depth == 0 => {
                    result.push(s[start..i].trim());
                    start = i + 1;
                }
                _ => {}
            }
        }

        if start < s.len() {
            let remainder = s[start..].trim();
            if !remainder.is_empty() {
                result.push(remainder);
            }
        }

        result
    }

    pub fn to_rust_type(&self) -> String {
        if let Some(x) = map_csharp_primitive(self.name) {
            return x.to_string()
        }

        // everything starts with crate:: to not have to do use crate:: stuff
        let mut res = "crate::".to_string();
        if !self.hierarchy.is_empty() {
            for item in &self.hierarchy[..self.hierarchy.len() - 1] {
                res.push_str(item.to_lowercase().as_str());
                res.push_str("::");
            }
            res.push_str(&clean_name(self.hierarchy[self.hierarchy.len() - 1]));
        }
        res
    }

    pub fn qualified_struct_path(&self) -> String {
        if let Some(x) = map_csharp_primitive(self.name) {
            return x.to_string()
        }
        let mut res = "crate::".to_string();
        if !self.hierarchy.is_empty() {
            for item in &self.hierarchy[..self.hierarchy.len() - 1] {
                res.push_str(item.to_lowercase().as_str());
                res.push_str("::");
            }
            res.push_str(&clean_name(self.hierarchy[self.hierarchy.len() - 1]));
        }
        res
    }

    pub fn struct_name(&self) -> String {
        if let Some(last) = self.hierarchy.last() {
            return clean_name(last)
        }
        clean_name(self.name)
    }
}

pub fn get_field_size<'a>(field: &REField<'a>, ty: &REType<'a>) -> u32 {
    if ty.parent == "System.Enum" || ty.parent == "System.ValueType" {
        if field.flags.contains(&REFieldFlag::Pointer) || field.flags.contains(&REFieldFlag::PointerOrRef) {
            0x8
        } else {
            ty.size.saturating_sub(0x10)
        }
    } else {
        0x8
    }
}

// remove the array brackets from the end of the name since we wrap it ourselves with Array<T>
pub fn clean_name(name: &str) -> String {
    name.replace(['`','<','>'], "_").replace(['[',']'], "")
}

pub fn format_generated_code(output_dir: &Path) {
    log::info!("Formatting generated code...");

    let status = Command::new("cargo")
        .arg("fmt")
        .current_dir(output_dir)
        .status()
        .expect("Failed to execute cargo fmt");

    if !status.success() {
        log::warn!("cargo fmt failed to format some files.");
    } else {
        log::info!("Formatting complete!");
    }
}

pub fn map_csharp_primitive(cs_type: &str) -> Option<&'static str> {
    match cs_type {
        "System.Boolean" => Some("bool"),
        "System.Byte"    => Some("u8"),
        "System.SByte"   => Some("i8"),
        "System.Char"    => Some("u16"),
        "System.Int16"   => Some("i16"),
        "System.UInt16"  => Some("u16"),
        "System.Int32"   => Some("i32"),
        "System.UInt32"  => Some("u32"),
        "System.Int64"   => Some("i64"),
        "System.UInt64"  => Some("u64"),
        "System.Single"  => Some("f32"),
        "System.Double"  => Some("f64"),
        "System.IntPtr"  => Some("isize"),
        "System.UIntPtr" => Some("usize"),
        _ => None,
    }
}
