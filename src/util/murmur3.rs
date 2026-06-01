#[inline]
fn murmur_32_scramble(k: u32) -> u32 {
    let mut k = k.wrapping_mul(0xcc9e2d51);
    k = (k << 15) | (k >> 17);
    k = k.wrapping_mul(0x1b873593);
    k
}

pub fn murmur3_32(key: &[u8], seed: u32) -> u32 {
    let mut h = seed;
    let len = key.len();
    let n_blocks = len / 4;

    for i in 0..n_blocks {
        let start = i * 4;
        let b = &key[start..start + 4];
        let k = u32::from_le_bytes(b.try_into().unwrap());
        h ^= murmur_32_scramble(k);
        h = h.rotate_left(13);
        h = h.wrapping_mul(5).wrapping_add(0xe6546b64);
    }

    let tail = &key[n_blocks * 4..];
    let mut k = 0u32;
    if tail.len() >= 3 { k ^= (tail[2] as u32) << 16; }
    if tail.len() >= 2 { k ^= (tail[1] as u32) << 8; }
    if tail.len() >= 1 { 
        k ^= tail[0] as u32;
        h ^= murmur_32_scramble(k); 
    }

	h ^= len as u32;
	h ^= h >> 16;
	h = h.wrapping_mul(0x85ebca6b);
	h ^= h >> 13;
	h = h.wrapping_mul(0xc2b2ae35);
	h ^= h >> 16;
    h
}

#[inline(always)]
pub fn murmur3_n(key: impl AsRef<[u8]>, seed: u32) -> u32 {
    murmur3_32(key.as_ref(), seed)
}

#[inline(always)]
pub fn murmur3(key: impl AsRef<[u8]>) -> u32 {
    murmur3_32(key.as_ref(), 0xffffffff)
}
