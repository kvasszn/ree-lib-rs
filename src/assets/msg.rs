use std::io::{Cursor, Read, Seek, SeekFrom};

use byteorder::{LE, ReadBytesExt};
use serde::{Deserialize, Serialize};

use crate::language::Language;
use crate::rsz::rsz_type::RszType;
use crate::assets::error::{FileReadError, Result};
use crate::types::{Guid, StringU16C};

const KEY: [u8; 16] = [
    207, 206, 251, 248, 236, 10, 51, 102, 147, 169, 29, 147, 80, 57, 95, 9,
];

pub struct MsgContext<'a, 'b> {
    pub data_offset: u64,
    pub lang_count: u32,
    pub attr_count: u32,
    pub cursor: &'a mut Cursor<&'b [u8]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MsgAttribute {
    Int(i64),
    Float(f64),
    String(String),
    Unknown(u64),
}

impl MsgAttribute {
    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self> {
        let attr = reader.read_u64::<LE>()?;
        Ok(Self::Int(attr as i64))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct MsgEntry {
    pub guid: Guid,
    pub unk: u32,
    pub hash: u32,
    pub name: String,
    pub attributes_offset: u64,
    pub content: Vec<String>,
    pub attributes: Vec<MsgAttribute>,
}

impl MsgEntry {
    fn read<R: Read + Seek>(reader: &mut R, lang_count: usize, attr_count: usize, data_offset: usize) -> Result<Self> {
        let guid: Guid = Guid::read_rsz(reader)?;
        let unk = reader.read_u32::<LE>()?;
        let hash = reader.read_u32::<LE>()?;
        let name = StringU16C::read_rsz(reader)?.to_string();
        let attributes_offset = reader.read_u64::<LE>()?;

        let mut content = Vec::with_capacity(lang_count as usize);
        for _ in 0..lang_count {
            content.push(StringU16C::read_rsz(reader)?.as_string());
        }

        let pos = reader.stream_position()?;
        reader.seek(SeekFrom::Start(attributes_offset))?;

        let mut attributes = Vec::with_capacity(attr_count as usize);
        for _ in 0..attr_count {
            attributes.push(MsgAttribute::read(reader)?);
        }

        reader.seek(SeekFrom::Start(pos))?;

        Ok(MsgEntry {
            guid,
            unk,
            hash,
            name,
            attributes_offset,
            content,
            attributes,
        })
    }
}

fn decrypt_msg_data(buf: &mut [u8]) {
    let mut prev_byte = 0;
    for (b, &key_byte) in buf.iter_mut().zip(KEY.iter().cycle()) {
        let cur = *b;
        *b = prev_byte ^ cur ^ key_byte;
        prev_byte = cur;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct MsgFile {
    pub version: u32,
    pub languages: Vec<u32>,
    pub p: u64,
    pub attr_types: Vec<i32>,
    pub attr_names: Vec<String>,
    pub entries: Vec<MsgEntry>,
}

impl MsgFile {
    pub fn read(data: &[u8]) -> super::error::Result<Self> {
        let mut reader = Cursor::new(data);

        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        if &magic != b"GMSG" {
            return Err(FileReadError::InvalidMagic(*b"GMSG", magic))
        }
        let version = reader.read_u32::<LE>()?;
        let _header_offset = reader.read_u64::<LE>()?;
        let entry_count = reader.read_u32::<LE>()?;
        let attr_count = reader.read_u32::<LE>()?;
        let lang_count = reader.read_u32::<LE>()?;
        let _null = reader.read_u32::<LE>()?;
        let data_offset = reader.read_u64::<LE>()?;
        let p_offset = reader.read_u64::<LE>()?;
        let lang_offset = reader.read_u64::<LE>()?;
        let attr_type_offset = reader.read_u64::<LE>()?;
        let attr_type_name_offset = reader.read_u64::<LE>()?;

        let mut entry_offsets = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            entry_offsets.push(reader.read_u64::<LE>()?);
        }

        reader.seek(SeekFrom::Start(lang_offset))?;
        let mut languages = Vec::with_capacity(lang_count as usize);
        for _ in 0..lang_count {
            languages.push(reader.read_u32::<LE>()?);
        }

        reader.seek(SeekFrom::Start(p_offset))?;
        let p = reader.read_u64::<LE>()?;

        reader.seek(SeekFrom::Start(attr_type_offset))?;
        let mut attr_types = Vec::with_capacity(attr_count as usize);
        for _ in 0..attr_count {
            attr_types.push(reader.read_i32::<LE>()?);
        }

        let mut data = reader.into_inner()[data_offset as usize..].to_vec();
        decrypt_msg_data(&mut data);

        let mut reader = Cursor::new(data);

        reader.seek(SeekFrom::Start(attr_type_name_offset - data_offset))?;
        let mut attr_names = Vec::with_capacity(attr_count as usize);
        for _ in 0..attr_count {
            attr_names.push(StringU16C::read_rsz(&mut reader)?.as_string());
        }

        let mut entries = Vec::with_capacity(entry_count as usize);
        for offset in &entry_offsets {
            reader.seek(SeekFrom::Start(*offset - data_offset))?;
            entries.push(MsgEntry::read(&mut reader, lang_count as usize, attr_count as usize, data_offset as usize)?);
        }

        Ok(MsgFile {
            version, languages, p, attr_types, attr_names, entries,
        })
    }

    pub fn get_entry<'a>(&'a self, guid: &'a Guid, language: Language) -> Option<&'a str> {
        if let Some(entry) = self.entries.iter().find(|e| e.guid.0 == guid.0) {
            return entry.content.get(language as usize).map(|x| x.as_str())
        }
        log::debug!("Could not find entry for guid {}, {language:?}", guid.to_string());
        None
    }
}

