pub mod murmur3;
use byteorder::{ReadBytesExt};
pub use murmur3::*;

use serde::{Deserialize, Deserializer, de};
use std::ops::{Add, Sub, Rem};
use std::{collections::HashMap, io};
use std::borrow::Cow;

pub fn parse_address_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let clean_str = s.trim_start_matches("0x");
    u64::from_str_radix(clean_str, 16).map_err(serde::de::Error::custom)
}

pub fn parse_address_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let clean_str = s.trim_start_matches("0x");
    u32::from_str_radix(clean_str, 16).map_err(serde::de::Error::custom)
}

pub fn parse_hex_map<'de, D, V>(deserializer: D) -> Result<HashMap<u32, V>, D::Error>
where
    D: Deserializer<'de>,
    V: Deserialize<'de>,
{
    let temp_map = HashMap::<Cow<'de, str>, V>::deserialize(deserializer)?;
    let mut final_map = HashMap::with_capacity(temp_map.len());
    for (k, v) in temp_map {
        let clean_k = k.trim_start_matches("0x");
        let key = u32::from_str_radix(clean_k, 16)
            .map_err(de::Error::custom)?;
        final_map.insert(key, v);
    }
    Ok(final_map)
}

use bytemuck::Pod;
use std::io::{Read, Seek, Write};

pub fn read_pod<T: Pod, R: Read>(r: &mut R) -> io::Result<T> {
    let mut buf = vec![0u8; std::mem::size_of::<T>()];
    r.read_exact(&mut buf)?;
    Ok(bytemuck::cast_slice(&buf)[0])
}

pub fn write_pod<T: Pod, W: Write>(w: &mut W, val: &T) -> io::Result<()> {
    let bytes = bytemuck::bytes_of(val);
    w.write_all(bytes)
}

pub fn read_pod_vec<T: Pod, R: Read>(r: &mut R, n: usize) -> io::Result<Vec<T>> {
    let mut vec = vec![T::zeroed(); n];
    let byte_slice = bytemuck::cast_slice_mut(&mut vec);
    r.read_exact(byte_slice)?;
    Ok(vec)
}

pub fn write_pod_vec<T: Pod, W: Write>(w: &mut W, val: &[T]) -> io::Result<()> {
    let byte_slice = bytemuck::cast_slice(val);
    w.write_all(byte_slice)
}

pub fn align_up<T: Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>>(
    value: T, align: T,
) -> T {
    value + (align - value % align) % align
}

pub fn seek_align_up<S: Seek + ?Sized>(stream: &mut S, align: u64) -> std::io::Result<u64> {
    let pos = stream.stream_position()?;
    let aligned = align_up(pos, align);
    if aligned != pos {
        stream.seek(std::io::SeekFrom::Start(aligned))?;
    }
    Ok(aligned)
}

pub trait ReadStringExt {
    fn read_u8str(&mut self) -> io::Result<String>;
}

impl<R: Read + ?Sized> ReadStringExt for R {
    fn read_u8str(&mut self) -> io::Result<String> {
        let mut s = Vec::new();
        loop {
            let c = self.read_u8()?;
            if c == 0 {
                break;
            }
            s.push(c);
        }
        String::from_utf8(s).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid UTF-8 sequence: {e}")
            )
        })
    }
}
