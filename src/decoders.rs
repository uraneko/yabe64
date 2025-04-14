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
pub struct Decoder;

impl Decoder {
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
    pub fn decode(value: impl AsRef<str>) -> Result<String, DecodeError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Ok("".into());
        }

        let base = match Self::deduce_encoding(value) {
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
    // 0 - if base encodings had patterns
    // then encoding can be further deduced through patterns
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
        } else if value
            .chars()
            .all(|c| ('0'..='9').contains(&c) || ('A'..='F').contains(&c))
        {
            if len % 2 == 0 {
                return Ok(BASE16);
            } else if len % 3 == 1 {
                return Err(DecodeError::BadEncodedString);
            }
        } else if value.chars().all(|c| {
            ('0'..='9').contains(&c)
                || ('A'..='Z').contains(&c)
                || [' ', '$', '%', '*', '+', '-', '.', '/', ':'].contains(&c)
        }) {
            let residual = len % 3;
            if residual != 1 {
                return Ok(BASE45);
            } else if len % 8 != 0 {
                return Err(DecodeError::BadEncodedString);
            }
        }
        // HACK this function needs a refactor... again
        // should have been if else not if
        if value
            .chars()
            .all(|c| ('0'..='9').contains(&c) || ('A'..='V').contains(&c) || c == '=')
        {
            return Ok(BASE32HEX);
        } else if !value.contains(['0', '1', '8', '9'])
        /* || value.contains(['W', 'X', 'Y', 'Z']) */ // this should be an and maybe condition
        {
            return Ok(BASE32);
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
mod bench_deduce_encoding {
    extern crate test;
    use test::Bencher;

    // BUG this string breaks guess_encoding for base32hex
    const DATA: &str = "io8yyioljb";

    // BUG this string breaks deduce_encoding for base45
    const DATA2: &str = "*IHO";

    // BUG breaks guess_encoding for base32hex and base16

    // NOTE
    // new deduce function
    // make function more robust
    // increased performance
    // fixed a bug where encoding cant be deduced for 32 hex encoding
    // but instead of deducing correctly (32hex) it now deduces to 32
    // this can't be helped as there are no chars from the extended hex table
    // that can allow for the deduction of the base as 32hex and not 32
    // for now, in such cases, use force_decode
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
}
