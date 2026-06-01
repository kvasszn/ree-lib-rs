use std::{
    collections::{HashMap}
};

use rustc_hash::FxHashMap;

use crate::util::*;

use anyhow::{Result};
use serde::{Deserialize, Deserializer};
use strum::EnumString;


fn deserialize_fields<'de, 'a, D>(deserializer: D) -> Result<FxHashMap<&'a str, REField<'a>>, D::Error>
where
    D: Deserializer<'de>,
    'de: 'a
{
    let mut map: FxHashMap<&'a str, REField<'a>> = HashMap::deserialize(deserializer)?;
    for (key, t_data) in map.iter_mut() {
        t_data.name = key;
    }
    Ok(map)
}

fn deserialize_methods<'de, 'a,  D>(deserializer: D) -> Result<FxHashMap<&'a str, REMethod<'a>>, D::Error>
where
    D: Deserializer<'de>,
    'de: 'a
{
    let mut map: FxHashMap<&'a str, REMethod<'a>> = HashMap::deserialize(deserializer)?;
    for (key, t_data) in map.iter_mut() {
        t_data.name = key;
    }
    Ok(map)
}

pub fn deserialize_il2cpp<'de, 'a, D>(deserializer: D) -> Result<Il2Cpp<'a>, D::Error>
where
    D: Deserializer<'de>,
    'de: 'a
{
    let mut map: FxHashMap<&'a str, REType<'a>> = HashMap::deserialize(deserializer)?;
    for (key, t_data) in map.iter_mut() {
        t_data.name = key;
    }
    Ok(map)
}

pub type Il2Cpp<'a> = FxHashMap<&'a str, REType<'a>>;

// i really should impl Deserialize myself to get better descriptions of the type,
// or just convert to PDBType, etc before adding everything
#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct REType<'a> {
    #[serde(skip)]
    pub name: &'a str,
    #[serde(default, deserialize_with = "parse_address_u64")]
    pub address: u64,
    #[serde(deserialize_with = "parse_address_u32")]
    crc: u32,
    #[serde(default, deserialize_with = "deserialize_type_flags")]
    pub flags: Vec<RETypeFlag>,
    #[serde(default, deserialize_with = "deserialize_fields")]
    pub fields: FxHashMap<&'a str, REField<'a>>,
    #[serde(deserialize_with = "parse_address_u64")]
    fqn: u64,
    #[serde(default)]
    id: u64,
    #[serde(default)]
    pub is_generic_type: bool,
    #[serde(default)]
    is_generic_type_definition: bool,
    #[serde(default, deserialize_with = "deserialize_methods")]
    pub methods: FxHashMap<&'a str, REMethod<'a>>,
    #[serde(default)]
    name_hierarchy: Vec<&'a str>,
    #[serde(default)]
    pub native_typename: &'a str,
    #[serde(default)]
    pub parent: &'a str,
    #[serde(default)]
    pub properties: FxHashMap<&'a str, REProperty<'a>>,
    #[serde(default, deserialize_with = "parse_address_u32")]
    pub size: u32,
}

#[derive(Debug, PartialEq, EnumString)]
pub enum RETypeFlag {
    ContainsGenericParameters,
    Finalize,
    NativeCtor,
    NativeType,
    ClassSemanticsMask,
    Public,
    NestedPublic,
    NestedFamily,
    SequentialLayout,
    ExplicitLayout,
    Sealed,
    Private,
    Abstract,
    Serializable,
    BeforeFieldInit,
    LocalHeap,
    ManagedVTable
}

fn deserialize_type_flags<'de, D>(deserializer: D) -> Result<Vec<RETypeFlag>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = <&str>::deserialize(deserializer)?;
    let flags: Vec<RETypeFlag> = value.split('|')
        .filter_map(|s| {
            let s = s.trim();
            match s.parse::<RETypeFlag>() {
                Ok(flag) => Some(flag),
                Err(_) => {
                    eprintln!("[WARN] Unknown RETypeFlag: {s:?}");
                    None
                }
            }
        })
    .collect();
    //let flags = flags.map_err(serde::de::Error::custom)?;
    Ok(flags)
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct REField<'a> {
    #[serde(skip)]
    pub name: &'a str,
    id: u64,
    pub init_data_index: u32,
    #[serde(deserialize_with = "parse_address_u32")]
    pub offset_from_base: u32,
    #[serde(deserialize_with = "parse_address_u32")]
    pub offset_from_fieldptr: u32,
    pub r#type: &'a str,
    #[serde(default, deserialize_with = "deserialize_field_flags")]
    pub flags: Vec<REFieldFlag>,
}


#[derive(Debug, PartialEq, EnumString)]
pub enum REFieldFlag {
    PointerOrRef,
    Pointer,
    Private,
    NotSerialized,
    FamANDAssem,
    Family,
    Static,
    InitOnly,
    HasFieldRVA,
    ExposeMember,
    Literal,
    SpecialName,
    RTSpecialName,
    HasDefault,
}

fn deserialize_field_flags<'de, D>(deserializer: D) -> Result<Vec<REFieldFlag>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = <&str>::deserialize(deserializer)?;
    let flags: Vec<REFieldFlag> = value.split('|')
        .filter_map(|s| {
            let s = s.trim();
            match s.parse::<REFieldFlag>() {
                Ok(flag) => Some(flag),
                Err(_) => {
                    eprintln!("[WARN] Unknown REFieldFlag: {s:?}");
                    None
                }
            }
        })
    .collect();
    //let flags = flags.map_err(serde::de::Error::custom)?;
    Ok(flags)
}


#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct REMethod<'a> {
    #[serde(skip)]
    pub name: &'a str,
    #[serde(default, deserialize_with = "deserialize_method_flags")]
    pub flags: Vec<REMethodFlag>,
    #[serde(deserialize_with = "parse_address_u64")]
    pub function: u64,
    pub id: u64,
    #[serde(default, deserialize_with = "deserialize_impl_flags")]
    pub impl_flags: Vec<REImplFlag>,
    pub invoke_id: u32,
    #[serde(borrow)]
    pub params: Option<Vec<REParam<'a>>>,
    pub returns: Option<REParam<'a>>,
    pub vtable_index: Option<u32>,
}

#[derive(Debug, PartialEq, EnumString)]
pub enum REMethodFlag {
    Private,
    FamANDAssem,
    Final,
    Family,
    Static,
    Virtual,
    HideBySig,
    SpecialName,
    RTSpecialName,
    NewSlot,
    Abstract,
    NoInlining,
}

fn deserialize_method_flags<'de, D>(deserializer: D) -> Result<Vec<REMethodFlag>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = <&str>::deserialize(deserializer)?;
    let flags = value.split('|')
        .filter_map(|s| {
            let s = s.trim();
            match s.parse::<REMethodFlag>() {
                Ok(flag) => Some(flag),
                Err(_) => {
                    eprintln!("[WARN] Unknown REMethodFlag: {s:?}");
                    None
                }
            }
        })
        .collect();
    Ok(flags)
}

#[derive(Debug, PartialEq, EnumString)]
pub enum REImplFlag {
    AggressiveOptimization,
    EmptyCtor,
    ExposeMember,
    NoInlining,
    Native,
    HasRetVal,
    InternalCall,
    HasThis,
    ContainsGenericParameters,
    AggressiveInlining,
}

fn deserialize_impl_flags<'de, D>(deserializer: D) -> Result<Vec<REImplFlag>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = <&str>::deserialize(deserializer)?;
    let flags: Vec<REImplFlag> = value.split('|')
        .filter_map(|s| {
            let s = s.trim();
            match s.parse::<REImplFlag>() {
                Ok(flag) => Some(flag),
                Err(_) => {
                    eprintln!("[WARN] Unknown REImplFlag: {s:?}");
                    None
                }
            }
        })
    .collect();
    //let flags = flags.map_err(serde::de::Error::custom)?;
    Ok(flags)
}


#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct REParam<'a> {
    pub name: &'a str,
    pub r#type: &'a str,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct REProperty<'a> {
    #[serde(skip)]
    pub name: &'a str,
    pub getter: &'a str,
    pub id: u32,
    pub setter: &'a str,
}

