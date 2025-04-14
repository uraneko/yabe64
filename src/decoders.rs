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
    BadEncodedString,
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

    // deduction methods
    // 1 - length of value should be different between bases
    // 2 - existence of padding char '='
    // 3 - the char ranges, some chars are specific to some base(s) alphabets
    // 0 - b32 and b32hex give me 2 different encoded strings
    // ofc since their tables are vastly different,
    // doesnt matter, as the original strings could be anything
    /// Deduces the string encoding by process of elimination. Takes a base encoded string.
    ///
    /// # Return
    ///
    /// returns an `Ok(Base)` if no errors were found and a base was guessed safely, or an `Err(DecodeError)` if:
    ///
    /// * a base was deduced but string contains char(s) that don't belong to that base table
    /// * a base couldn't be deduced
    pub fn deduce_encoding(value: &str) -> Result<Base, DecodeError> {
        let len = value.len();
        if value.contains(char::is_lowercase) {
            if len % 4 != 0 {
                return Err(DecodeError::BadEncodedString);
            }

            // NOTE: even if the actual encoding is base64url
            // if no base64url specific chars are found
            // then it can be treated as normal base64
            return Ok(if value.contains(['_', '-']) {
                BASE64URL
            } else {
                BASE64
            });
        } else if value.contains([' ', '$', '%', '*', '+', '-', '.', '/', ':']) {
            let residual = len % 3;
            if residual == 1 {
                return Err(DecodeError::BadEncodedString);
            }

            return Ok(BASE45);
        } else if value
            .chars()
            .all(|c| ('0'..='9').contains(&c) || ('A'..='F').contains(&c))
        {
            if len % 2 != 0 {
                return Err(DecodeError::BadEncodedString);
            }

            return Ok(BASE16);
        } else if value.contains(['0', '1', '8', '9']) && !value.contains(['W', 'X', 'Y', 'Z']) {
            if len % 8 != 0 {
                return Err(DecodeError::BadEncodedString);
            }

            return Ok(BASE32HEX);
        } else if !value.contains(['0', '1', '8', '9']) || value.contains(['W', 'X', 'Y', 'Z']) {
            if len % 8 != 0 {
                return Err(DecodeError::BadEncodedString);
            }

            return Ok(BASE32);
        }

        Err(DecodeError::UnknownBaseEncodingIfAny)
    }

    // DEPRECATED
    // #[deprecated(since = "0.1.2", note = "please use deduce_encoding instead")]
    pub fn guess_encoding(&self, value: &str) -> Result<Base, DecodeError> {
        let len = value.len();
        let chars = value.chars();
        let is_64 = chars.clone().any(|c| c.is_ascii_lowercase())
            || chars.clone().any(|c| ['+', '/', '-', '_'].contains(&c));
        let is_64 = is_64 && len % 4 == 0;
        if is_64 {
            if chars.clone().any(|c| !is_base64(c)) {
                return Err(DecodeError::BadEncodedString);
            }

            match value.contains(['-', '_']) {
                // base 64 url decode
                true => {
                    return if chars.clone().any(|c| !is_base64_url(c)) {
                        Err(DecodeError::BadEncodedString)
                    } else {
                        Ok(Base::_64URL)
                    };
                }
                // base 64 decode
                false => {
                    return if chars.clone().any(|c| !is_base64_normal(c)) {
                        Err(DecodeError::BadEncodedString)
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

#[cfg(feature = "nightly")]
// TODO: fix this; use test/bench api
// this module benchmarks different versions of the deduce_encoding Decoder function
mod bench_decode_encoding {
    extern crate test;
    use test::Bencher;

    const DATA: &str = "io8yyioljb";

    // NOTE
    // new deduce function
    // increased performance
    // fixed a bug where encoding cant be deduced for 32 hex encoding
    // but instead of deducing correctly (32hex) it now deduces to 32
    // this can't be helped as there are no chars from the extended hex table
    // that can allow for the deduction of the base as 32hex and not 32
    #[bench]
    fn bench_deduce_012(b: &mut Bencher) {
        let encs = [
            crate::Encoder::base64().encode(DATA),
            crate::Encoder::base64_url().encode(DATA),
            crate::Encoder::base45().encode(DATA),
            crate::Encoder::base32().encode(DATA),
            crate::Encoder::base32_hex().encode(DATA),
            crate::Encoder::base16().encode(DATA),
        ];
        b.iter(|| {
            encs.iter().for_each(|e| {
                crate::Decoder::deduce_encoding(&e).unwrap();
            })
        });
    }

    #[bench]
    fn bench_guess_011(b: &mut Bencher) {
        let encs = [
            crate::Encoder::base64().encode(DATA),
            crate::Encoder::base64_url().encode(DATA),
            crate::Encoder::base45().encode(DATA),
            crate::Encoder::base32().encode(DATA),
            crate::Encoder::base32_hex().encode(DATA),
            crate::Encoder::base16().encode(DATA),
        ];
        let dec = crate::Decoder::new();

        b.iter(|| {
            encs.iter().for_each(|e| {
                dec.guess_encoding(&e).unwrap();
            })
        });
    }
}
