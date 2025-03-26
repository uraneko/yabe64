use super::DecodeError;
use crate::char_checks::*;
use crate::{Base, PAD, idx_from_char};

const BASE64: Base = Base::_64;
const BASE64URL: Base = Base::_64URL;

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

// deduces the string encoding by process of elimination
fn guess_encoding(value: &str) -> Result<Base, DecodeError> {
    let chars = value.chars();
    let is_64 = chars.clone().any(|c| c.is_ascii_lowercase())
        || chars.clone().any(|c| ['+', '/', '-', '_'].contains(&c));
    if is_64 {
        if chars.clone().any(|c| !is_base64(c)) {
            return Err(DecodeError::EncodedStringIsCorrupt);
        }

        match value.contains(['-', '_']) {
            // base 64 url decode
            true => {
                return if chars.clone().any(|c| !is_base64_url(c)) {
                    println!("64url");
                    Err(DecodeError::EncodedStringIsCorrupt)
                } else {
                    Ok(Base::_64URL)
                };
            }
            // base 64 decode
            false => {
                return if chars.clone().any(|c| !is_base64_normal(c)) {
                    println!("64");
                    Err(DecodeError::EncodedStringIsCorrupt)
                } else {
                    Ok(Base::_64)
                };
            }
        }
    }

    let is_32 = chars.clone().any(|c| ['W', 'X', 'Y', 'Z'].contains(&c));
    if is_32 {
        if chars.clone().any(|c| !is_base32(c)) {
            println!("32");
            return Err(DecodeError::EncodedStringIsCorrupt);
        }

        return Ok(Base::_32);
    }

    let is_32_hex = !is_32 && chars.clone().any(|c| ['0', '1', '8', '9'].contains(&c));
    if is_32_hex {
        if chars.clone().any(|c| !is_base32_hex(c)) {
            println!("32hex");
            return Err(DecodeError::EncodedStringIsCorrupt);
        }

        return Ok(Base::_32HEX);
    }

    let is_16 = chars.clone().all(|c| c.is_ascii_hexdigit());
    if is_16 {
        println!("16");
        if chars.clone().any(|c| !is_base32_hex(c)) {
            return Err(DecodeError::EncodedStringIsCorrupt);
        }

        return Ok(Base::_16);
    }

    Err(DecodeError::StringIsNotBaseEncoded)
}

// turns back chars from the encoding table to their table index values
fn into_table_idx(value: &str, base: &Base) -> Vec<u8> {
    // no need for chars count, len is sufficient since all chars are ascii (1 byte)
    value
        .chars()
        .map(|c| match c {
            '=' => 0,
            val => idx_from_char(val, base),
        })
        .collect::<Vec<u8>>()
}

fn into_24bits_bytes(value: Vec<u8>) -> Vec<u32> {
    // NOTE len must be an integra multiple of 4
    value
        .chunks(4)
        .inspect(|c| println!("{:?}", c))
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

fn into_decoded(value: Vec<u8>) -> String {
    value.into_iter().map(|c| c as char).collect()
}

pub fn base64_decode<T>(value: T) -> String
where
    T: AsRef<str> + Into<String>,
{
    let value = value.as_ref();
    if value.is_empty() {
        return "".into();
    }

    let base = guess_encoding(value);
    if let Err(e) = base {
        panic!("{:?}", e);
    }
    let base = base.unwrap();
    let indices = into_table_idx(value, &base);
    let bytes = into_24bits_bytes(indices);
    let bytes = into_8bits_bytes(bytes);

    into_decoded(bytes)
}
