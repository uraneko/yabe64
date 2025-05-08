#![cfg(any(feature = "base64", feature = "base64_url"))]
use crate::makura_alloc::Vec;

use super::DecodeError;

/// DOCS
/// last 3 octets
/// (1) The final quantum of encoding input is an integral multiple of 24
///     bits; here, the final unit of encoded output will be an integral
///     multiple of 4 characters with no "=" padding.
///
/// (2) The final quantum of encoding input is exactly 8 bits; here, the
///     final unit of encoded output will be two characters followed by
///     two "=" padding characters.
///
/// (3) The final quantum of encoding input is exactly 16 bits; here, the
///     final unit of encoded output will be three characters followed by
///     one "=" padding character.

// to implement the other decoders
// only a different version of this function is needed
// the other functions stay the same
fn into_24bits_bytes(value: Vec<u8>) -> Vec<u32> {
    // NOTE len must be an integra multiple of 4
    value
        .chunks(4)
        // .inspect(|c| println!("{:?}", c))
        .map(|b| {
            let mut mask = 0u32;
            mask |= b[0] as u32;
            mask <<= 6;
            mask |= b[1] as u32;
            mask <<= 6;
            mask |= b[2] as u32;
            mask <<= 6;
            mask |= b[3] as u32;

            mask
        })
        .collect()
}

// get back 8 bit bytes from the 24bits bytes
fn into_8bits_bytes(value: Vec<u32>) -> Vec<u8> {
    let mut bytes = value
        .into_iter()
        .map(|b| {
            [
                ((b & 0xff0000) >> 16) as u8,
                ((b & 0xff00) >> 8) as u8,
                b as u8,
            ]
        })
        .flatten()
        .collect::<Vec<u8>>();
    while let Some(0) = bytes.last() {
        bytes.pop();
    }

    bytes
}

#[cfg(feature = "base64")]
pub fn base64_decode(indices: Vec<u8>) -> Vec<u8> {
    let bytes = into_24bits_bytes(indices);

    into_8bits_bytes(bytes)
}

#[cfg(feature = "base64_url")]
pub fn base64_url_decode(indices: Vec<u8>) -> Vec<u8> {
    let bytes = into_24bits_bytes(indices);

    into_8bits_bytes(bytes)
}
