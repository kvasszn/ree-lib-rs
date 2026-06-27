use std::io::{Cursor, Read, Seek};

use bincode::{Decode, Encode};
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use crate::{assets::error::{FileReadError, Result}, rsz::{Rsz, RszMap, rsz_type::RszType}, types::StringU16C, util::{read_pod, read_pod_vec}};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct UserHeader {
    magic: [u8; 4],
    resource_count: u32,
    child_count: u32,
    padding: u32,
    resource_list_offset: u64,
    child_list_offset: u64,
    rsz_offset: u64,
    rsz_offset_cap: u64,
}

#[derive(Debug, Serialize, Deserialize, Decode, Encode)]
pub struct UserChild {
    pub hash: u32,
    pub name: String,
}

#[derive(Debug, Decode, Encode)]
pub struct UserFile {
    pub resource_names: Vec<String>,
    pub children: Vec<UserChild>,
    pub rsz: Rsz,
}

impl UserFile {
    pub fn read<R: Read + Seek>(r: &mut R, rsz_map: &RszMap) -> Result<Self> {
        let UserHeader {
            magic,
            resource_count,
            child_count,
            //resource_list_offset,
            //child_list_offset,
            //rsz_offset,
            //rsz_offset_cap,
            ..
        } = read_pod::<UserHeader, R>(r)?;
        if &magic != b"USR\0" {
            return Err(FileReadError::InvalidMagic(*b"USR\0", magic));
        }

        let resource_name_offsets = read_pod_vec::<u64, R>(r, resource_count as usize)?;

        #[repr(C)]
        #[derive(Debug, Copy, Clone, Pod, Zeroable)]
        struct ChildInfo(u32, u32, u64);
        let child_info = read_pod_vec::<ChildInfo, R>(r, child_count as usize)?;

        let resource_names = resource_name_offsets.into_iter().map(|_offset| {
            let name = StringU16C::read_rsz(r)?.to_string();
            Ok(name)
        }).collect::<Result<Vec<String>>>()?;

        let children: Vec<UserChild> = child_info.into_iter().map(|ChildInfo(hash, _, _offset)| {
            let name = StringU16C::read_rsz(r)?.to_string();
            Ok(UserChild{hash, name})
        }).collect::<Result<_>>()?;

        let mut data = vec![];
        r.read_to_end(&mut data)?;
        let mut cursor = Cursor::new(data);
        let rsz = Rsz::read(&mut cursor, rsz_map)?;
        Ok(Self {
            resource_names,
            children,
            rsz
        })
    }
}
