#![cfg(any(feature = "base32", feature = "base32_hex"))]
use crate::makura_alloc::{String, Vec};

use crate::char_from_idx;
use crate::{BASE32, BASE32HEX};

/// DOCS
/// Special processing is performed if fewer than 40 bits are available
/// at the end of the data being encoded.  A full encoding quantum is
/// always completed at the end of a body.  When fewer than 40 input bits
/// are available in an input group, bits with value zero are added (on
/// the right) to form an integral number of 5-bit groups.  Padding at
/// the end of the data is performed using the "=" character.  Since all
/// base 32 input is an integral number of octets, only the following
/// cases can arise:
///
/// (1) The final quantum of encoding input is an integral multiple of 40
///     bits; here, the final unit of encoded output will be an integral
///     multiple of 8 characters with no "=" padding.
///
/// (2) The final quantum of encoding input is exactly 8 bits; here, the
///     final unit of encoded output will be two characters followed by
///     six "=" padding characters.
///
/// (3) The final quantum of encoding input is exactly 16 bits; here, the
///     final unit of encoded output will be four characters followed by
///     four "=" padding characters.
///
/// (4) The final quantum of encoding input is exactly 24 bits; here, the
///     final unit of encoded output will be five characters followed by
///     three "=" padding characters.
///
/// (5) The final quantum of encoding input is exactly 32 bits; here, the
///     final unit of encoded output will be seven characters followed by
///     one "=" padding character.

// separates the input string into chunks of 24bits
// bytes_of_u40
fn into_40bits_chunks(data: &str) -> Vec<u64> {
    let mut bytes = data.as_bytes().chunks(5);
    // println!("{:?}", bytes.clone().collect::<Vec<&[u8]>>());
    let last = bytes.next_back().unwrap();

    let mut bytes = bytes
        .map(|b| {
            let mut mask = 0u64;
            mask |= b[0] as u64;
            mask <<= 8;
            mask |= b[1] as u64;
            mask <<= 8;
            mask |= b[2] as u64;
            mask <<= 8;
            mask |= b[3] as u64;
            mask <<= 8;
            mask |= b[4] as u64;

            mask
        })
        .collect::<Vec<u64>>();
    let last = {
        let mut mask = 0u64;
        mask |= last[0] as u64;
        mask <<= 8;
        mask |= if last.len() < 2 { 0u64 } else { last[1] as u64 };
        mask <<= 8;
        mask |= if last.len() < 3 { 0u64 } else { last[2] as u64 };
        mask <<= 8;
        mask |= if last.len() < 4 { 0u64 } else { last[3] as u64 };
        mask <<= 8;
        mask |= if last.len() < 5 { 0u64 } else { last[4] as u64 };

        mask
    };
    bytes.push(last);

    bytes
}

// bytes_of_u5
fn into_5bits_bytes(bytes: Vec<u64>) -> Vec<u8> {
    let bytes = bytes.into_iter();
    // let mut last = bytes.next_back().unwrap();

    bytes
        .map(|b| {
            [
                // NOTE & 31 to take only the least 5 bits
                (b >> 35) as u8 & 31,
                (b >> 30) as u8 & 31,
                (b >> 25) as u8 & 31,
                (b >> 20) as u8 & 31,
                (b >> 15) as u8 & 31,
                (b >> 10) as u8 & 31,
                (b >> 5) as u8 & 31,
                b as u8 & 31,
            ]
        })
        .flatten()
        .collect()
}

fn into_base32(bytes: Vec<u8>) -> String {
    let bytes = bytes.into_iter();

    let mut cd = 6;
    let mut pad = true;
    bytes
        .rev()
        .map(|b| {
            if cd > 0 && pad && b == 0 {
                cd -= 1;
                '='
            } else {
                pad = false;
                char_from_idx(b, &BASE32)
            }
        })
        .collect::<Vec<char>>()
        .into_iter()
        .rev()
        .collect()
}

fn into_base32_hex(bytes: Vec<u8>) -> String {
    let bytes = bytes.into_iter();

    // FIXME the table needs to have all values
    let mut cd = 6;
    let mut pad = true;
    bytes
        .rev()
        // .inspect(|b| println!("{}", b))
        .map(|b| {
            if cd > 0 && pad && b == 0 {
                cd -= 1;
                '='
            } else {
                pad = false;
                char_from_idx(b, &BASE32HEX)
            }
        })
        .collect::<Vec<char>>()
        .into_iter()
        .rev()
        .collect()
}

#[cfg(feature = "base32")]
pub fn base32_encode<T>(value: T) -> String
where
    T: AsRef<str>,
{
    let value = value.as_ref();
    if value.is_empty() {
        return "".into();
    }

    let chunks = into_40bits_chunks(value);
    let bytes = into_5bits_bytes(chunks);

    into_base32(bytes)
}

#[cfg(feature = "base32_hex")]
pub fn base32_hex_encode<T>(value: T) -> String
where
    T: AsRef<str>,
{
    let value = value.as_ref();
    if value.is_empty() {
        return "".into();
    }

    let chunks = into_40bits_chunks(value);
    let bytes = into_5bits_bytes(chunks);

    into_base32_hex(bytes)
}
