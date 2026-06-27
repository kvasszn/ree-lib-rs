use std::io::{Read, Seek};
use bincode::{Decode, Encode};

use crate::{assets::error::{FileReadError, Result}};

#[derive(Decode, Encode)]
pub struct Oft {
    pub data: Vec<u8>,
}

impl Oft {
    pub fn read<R: Read + Seek>(r: &mut R) -> Result<Oft> {
        let mut magic = [0u8; 4];
        r.read(&mut magic)?;
        if &magic != b"FBFO" {
            return Err(FileReadError::InvalidMagic(*b"FBFO", magic));
        }

        let mut data = vec![];
        r.read_to_end(&mut data)?;

        // decrypt
        let mut seed = 1u64;
        let delta = 0xAE6E39B58A355F45u64;
        let size = data.len() & 0x3F;
        if size > 0 {
            for _ in 0..size {
                seed = 2 * seed + 1;
            }
        }

        let key = (delta >> size) | (seed & delta)  << (64 - size);
        let key_bytes = key.to_le_bytes();
        if data.len() > 0 {
            for i in 0..data.len() {
                data[i] ^= key_bytes[i % 8];
            }
        }

        Ok(Oft {
            data
        })
    }
}

