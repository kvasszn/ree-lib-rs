use std::io::{Cursor, Read, Seek, SeekFrom};

use bincode::{Decode, Encode};
use bytemuck::{Pod, Zeroable};
use byteorder::{LE, ReadBytesExt};
use serde::{Deserialize, Serialize};

use crate::assets::error::{FileReadError, Result};
use crate::language::Language;
use crate::rsz::rsz_type::RszType;
use crate::types::{Guid, StringU16C};
use crate::util::{read_pod, read_pod_vec};

const KEY: [u8; 16] = [
    207, 206, 251, 248, 236, 10, 51, 102, 147, 169, 29, 147, 80, 57, 95, 9,
];

#[derive(Debug, Clone, Serialize, Deserialize, Decode, Encode)]
pub enum MsgAttribute {
    Int(i64),
    Float(f64),
    String(String),
    Unknown(u64),
}

impl MsgAttribute {
    fn read<R: Read + Seek>(r: &mut R) -> Result<Self> {
        let attr = r.read_u64::<LE>()?;
        Ok(Self::Int(attr as i64))
    }
}

#[derive(Clone, Debug, Serialize, Decode, Encode)]
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
    fn read<R: Read + Seek>(
        r: &mut R,
        lang_count: usize,
        attr_count: usize,
    ) -> Result<Self> {
        let guid: Guid = Guid::read_rsz(r)?;
        let unk = r.read_u32::<LE>()?;
        let hash = r.read_u32::<LE>()?;
        let name_offset = r.read_u64::<LE>()?;
        let attributes_offset = r.read_u64::<LE>()?;
        let content_offsets = read_pod_vec::<u64, R>(r, lang_count)?;

        r.seek(SeekFrom::Start(name_offset))?;
        let name = StringU16C::read_rsz(r)?.to_string();

        r.seek(SeekFrom::Start(attributes_offset))?;
        let mut attributes = Vec::with_capacity(attr_count);
        for _ in 0..attr_count {
            attributes.push(MsgAttribute::read(r)?);
        }

        let mut content = Vec::with_capacity(lang_count);
        for offset in content_offsets {
            r.seek(SeekFrom::Start(offset))?;
            content.push(StringU16C::read_rsz(r)?.as_string());
        }

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

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct MsgHeader {
    pub version: u32,
    pub magic: [u8; 4],
    pub header_offset: u64,
    pub entry_count: u32,
    pub attr_count: u32,
    pub lang_count: u32,
    pub null: u32,
    pub data_offset: u64,
    pub p_offset: u64,
    pub lang_offset: u64,
    pub attr_type_offset: u64,
    pub attr_type_name_offset: u64,
}

#[derive(Clone, Debug, Serialize, Decode, Encode)]
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
        let mut r = Cursor::new(data);
        let header = read_pod(&mut r)?; 
        let MsgHeader {
            version,
            magic,
            header_offset,
            entry_count,
            attr_count,
            lang_count,
            null,
            data_offset,
            p_offset,
            lang_offset,
            attr_type_offset,
            attr_type_name_offset,
        } = header;
        if &magic != b"GMSG" {
            return Err(FileReadError::InvalidMagic(*b"GMSG", magic));
        }

        let entry_offsets = read_pod_vec::<u64, Cursor<&[u8]>>(&mut r, entry_count as usize)?;

        r.seek(SeekFrom::Start(lang_offset))?;
        let languages = read_pod_vec::<u32, Cursor<&[u8]>>(&mut r, lang_count as usize)?;

        r.seek(SeekFrom::Start(p_offset))?;
        let p = r.read_u64::<LE>()?;

        r.seek(SeekFrom::Start(attr_type_offset))?;
        let attr_types = read_pod_vec::<i32, Cursor<&[u8]>>(&mut r, attr_count as usize)?;

        r.seek(SeekFrom::Start(attr_type_name_offset))?;
        let attr_types_name_offsets = read_pod_vec::<u64, Cursor<&[u8]>>(&mut r, attr_count as usize)?;

        let mut data = r.into_inner().to_vec();
        decrypt_msg_data(&mut data[data_offset as usize..]);

        let mut r = Cursor::new(data);

        r.seek(SeekFrom::Start(attr_type_name_offset))?;
        let mut attr_names = Vec::with_capacity(attr_count as usize);
        for offset in attr_types_name_offsets {
            r.seek(SeekFrom::Start(offset))?;
            let s = StringU16C::read_rsz(&mut r)?.as_string();
            attr_names.push(s);
        }

        let mut entries = Vec::with_capacity(entry_count as usize);
        for offset in entry_offsets {
            r.seek(SeekFrom::Start(offset))?;
            entries.push(MsgEntry::read(
                &mut r,
                lang_count as usize,
                attr_count as usize,
            )?);
        }

        Ok(MsgFile {
            version,
            languages,
            p,
            attr_types,
            attr_names,
            entries,
        })
    }

    pub fn get_entry<'a>(&'a self, guid: &'a Guid, language: Language) -> Option<&'a str> {
        if let Some(entry) = self.entries.iter().find(|e| e.guid.0 == guid.0) {
            return entry.content.get(language as usize).map(|x| x.as_str());
        }
        log::debug!("Could not find entry for guid {}, {language:?}", guid);
        None
    }
}
