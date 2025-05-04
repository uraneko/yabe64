#![cfg(feature = "base16")]
use crate::makura_alloc::Vec;

fn into_8bits_bytes(value: Vec<u8>) -> Vec<u8> {
    value
        .chunks(2)
        .map(|b| {
            let mut mask = 0u8;
            mask |= b[0];
            mask <<= 4;
            mask |= b[1];

            mask
        })
        .collect()
}

pub fn base16_decode(indices: Vec<u8>) -> Vec<u8> {
    into_8bits_bytes(indices)
}
