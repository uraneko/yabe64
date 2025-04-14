#![cfg(feature = "decoding")]
use crate::makura_alloc::{String, Vec};

use super::char_checks::*;
use super::{Base, idx_from_char};

mod base16;
mod base32;
mod base45;
mod base64;

use base16::base16_decode;
use base32::base32_decode;
use base32::base32_hex_decode;
use base45::base45_decode;
use base64::base64_decode;
use base64::base64_url_decode;

use crate::{BASE16, BASE32, BASE32HEX, BASE45, BASE64, BASE64URL};

#[derive(Debug)]
pub enum DecodeError {
    /// string was deduced to be base<x> encoded but contains char(s) that
    /// don't belong to base<x>'s encoding table
    EncodedStringIsCorrupt,
    /// string encoding is not any of the implemented base encodings
    /// i.e., it is not base 64, 64url, 45, 32, 32hex or 16 encoded
    UnknownBaseEncodingIfAny,
}

// this only exists to match Encoder struct
// otherwise a free function works fine
pub struct Decoder {
    hint: Option<Base>,
}

impl Decoder {
    /// creates a new decoder
    pub fn new() -> Self {
        Self {
            hint: Default::default(),
        }
    }

    /// changes the Decoder's hint value to the passed base
    pub fn hint(mut self, base: Base) -> Self {
        self.hint = Some(base);

        self
    }

    /// decodes a given string
    /// takes encoded string and base of the string encoding
    /// returns decoded string value
    ///
    /// # Panic
    /// panics if the value string's actual encoding doesn't match the passed base
    ///
    /// * use this method when you know your input string's encoding for sure
    /// * otherwise, use decode method if not sure about the base encoding of the value string
    pub fn force_decode(value: impl AsRef<str>, base: Base) -> String {
        let value = value.as_ref();
        if value.is_empty() {
            return "".into();
        }

        match base {
            BASE64 => base64_decode(value),
            BASE64URL => base64_url_decode(value),
            BASE45 => base45_decode(value),
            BASE32 => base32_decode(value),
            BASE32HEX => base32_hex_decode(value),
            BASE16 => base16_decode(value),
        }
    }

    /// decodes the given string
    /// takes encoded string
    /// returns `Ok(String)` value of decoded string at success
    /// or `Err(DecodeError)` in case of failure
    ///
    /// # Error
    /// errors when `guess_encoding` returns an error
    ///
    /// otherwise always returns Ok
    pub fn decode(&self, value: impl AsRef<str>) -> Result<String, DecodeError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Ok("".into());
        }

        let base = match self.guess_encoding(value) {
            Err(e) => return Err(e),
            Ok(b) => b,
        };

        Ok(match base {
            BASE64 => base64_decode(value),
            BASE64URL => base64_url_decode(value),
            BASE45 => base45_decode(value),
            BASE32 => base32_decode(value),
            BASE32HEX => base32_hex_decode(value),
            BASE16 => base16_decode(value),
        })
    }

    /// Deduces the string encoding by process of elimination. Takes a base encoded string.
    ///
    /// # Return
    ///
    /// returns an `Ok(Base)` if no errors were found and a base was guessed safely, or an `Err(DecodeError)` if:
    ///
    /// * a base was deduced but string contains char(s) that don't belong to that base table
    /// * a base couldn't be deduced
    pub fn guess_encoding(&self, value: &str) -> Result<Base, DecodeError> {
        let len = value.len();
        let chars = value.chars();
        let is_64 = chars.clone().any(|c| c.is_ascii_lowercase())
            || chars.clone().any(|c| ['+', '/', '-', '_'].contains(&c));
        let is_64 = is_64 && len % 4 == 0;
        if is_64 {
            if chars.clone().any(|c| !is_base64(c)) {
                return Err(DecodeError::EncodedStringIsCorrupt);
            }

            match value.contains(['-', '_']) {
                // base 64 url decode
                true => {
                    return if chars.clone().any(|c| !is_base64_url(c)) {
                        Err(DecodeError::EncodedStringIsCorrupt)
                    } else {
                        Ok(Base::_64URL)
                    };
                }
                // base 64 decode
                false => {
                    return if chars.clone().any(|c| !is_base64_normal(c)) {
                        Err(DecodeError::EncodedStringIsCorrupt)
                    } else {
                        Ok(Base::_64)
                    };
                }
            }
        }

        let is_32 = chars.clone().all(|c| is_base32(c)) && len % 8 == 0;
        let is_32_hex = chars.clone().all(|c| is_base32_hex(c)) && len % 8 == 0;
        let is_16 = chars.clone().all(|c| c.is_ascii_hexdigit()) && len % 2 == 0;

        let is_45 = chars.clone().all(|c| is_base45(c)) && (len % 3 == 0 || len % 3 == 2);
        if is_45 && !is_32 && !is_32_hex && !is_16 {
            return Ok(Base::_45);
        }

        if is_32 && is_32_hex {
            if let Some(b) = &self.hint {
                return Ok(*b);
            }
        } else if is_32 {
            return Ok(Base::_32);
        } else if is_32_hex {
            return Ok(Base::_32HEX);
        } else if is_16 {
            return Ok(Base::_16);
        }

        Err(DecodeError::UnknownBaseEncodingIfAny)
    }
}

// turns back chars from the encoding table to their table index values
pub(self) fn into_table_idx(value: &str, base: &Base) -> Vec<u8> {
    // no need for chars count, len is sufficient since all chars are ascii (1 byte)
    value
        .chars()
        .map(|c| match c {
            '=' => 0,
            val => idx_from_char(val, base),
        })
        .collect::<Vec<u8>>()
}

pub(self) fn into_decoded(value: Vec<u8>) -> String {
    value.into_iter().map(|c| c as char).collect()
}
