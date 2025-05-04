#![cfg(feature = "decoding")]
use crate::makura_alloc::{String, Vec, vec};

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

use crate::makura_alloc::FromUtf8Error;
use crate::{BASE16, BASE32, BASE32HEX, BASE45, BASE64, BASE64URL};

/// errors that can occur during the decoding process of some base encoded input value
#[derive(Debug)]
pub enum DecodeError {
    /// string was deduced to be base<x> encoded but contains char(s) that
    /// don't belong to base<x>'s encoding table
    BadEncodedString,
    /// the length of the input value
    /// doesn't fit the value length that the given base encoding should generate
    /// e.g., all base64 encoded strings should have a length that
    /// satisfies len % 4 == 0
    ///
    /// .0 corresponds to the bad length value
    BadLenForBase(usize),
    /// string encoding is not any of the implemented base encodings
    /// i.e., it is not base 64, 64url, 45, 32, 32hex or 16 encoded
    UnknownBaseEncodingIfAny,
    /// when trying to decode some base encoded string's char using said base's alphabet table
    /// this variant is returned if the given char is not part of that base's alphabet table
    UnrecognizedCharForBase { ch: char, base: Base },
    // UnrecognizedIndexForBase {
    //     idx: u8,
    //     base: Base,
    // },
    /// one or more encoded input vec bytes have a value greater that the base encoding's
    /// table max value
    /// e.g., a base64 encoded string bytes should all satisfy 0 < byte <= 63
    TableIndexOverflow { base: Base, value: u8 },
    /// when decoding an encoded string that is supposed to be of base 16 or 45
    /// both of which can not contain padding '=' chars
    /// yet a padding char was found at the end of the encoded string
    BaseEncodingHasNoPaddingChars(Base),
    /// results from trying togenerate a string from a Vec<u8> decoded bytes of an
    /// originally encoded string value
    ///
    /// this variant simply passes on the error value from the alloc::string::String::from_utf8
    /// String method
    FromUtf8Error(FromUtf8Error),
    /// can only be reached from the deduce_exclude Decoder function
    /// signifies that the correct base was deduced but it has been excluded from the deduction
    /// the deduction process exits with this error value since further deduction is useless
    EncodingBaseIsExcluded(Base),
}

// this only exists to match Encoder struct
// otherwise a free function works fine
pub struct Decoder;

impl Decoder {
    pub fn into_string(value: Vec<u8>) -> Result<String, DecodeError> {
        let res = String::from_utf8(value);
        if res.is_ok() {
            res.map_err(|_| DecodeError::BadEncodedString)
        } else {
            res.map_err(|e| DecodeError::FromUtf8Error(e))
        }
    }

    // turns back chars from the encoding table to their table index values
    pub(self) fn into_table_idx(value: &str, base: &Base) -> Result<Vec<u8>, DecodeError> {
        // no need for chars count, len is sufficient since all chars are ascii (1 byte)
        // WARN they are not all ascii, baseless assumption
        // but i cant recall what the line above is talking about
        let val = value.chars().map(|c| match c {
            '=' => {
                if base == &BASE16 || base == &BASE45 {
                    Err(DecodeError::BaseEncodingHasNoPaddingChars(*base))
                } else {
                    Ok(0)
                }
            }
            val => idx_from_char(val, base),
        });
        if val.clone().any(|i| i.is_err()) {
            return Err(DecodeError::BadEncodedString);
        }

        Ok(val.map(|i| i.unwrap()).collect::<Vec<u8>>())
    }

    /// decodes a given string
    /// takes encoded string and user provided base of the string encoding
    ///
    /// returns a result of the decoded string value or a `DecodeError`
    ///
    /// # Error
    /// returns an Err when the inner decode function returns an error,
    /// which is when the passed encoded string and encoding base do not match
    ///
    /// * use this method when you know your input string's encoding for sure
    /// * otherwise, use decode_deduce method if not sure about the base encoding of the value string
    ///
    /// Note that `decode_deduce`'a deduction is not alawys correct
    // NOTE was force_decode
    // TODO all decode functions need to add assert_encoding
    // if it errors they error without decoding
    pub fn decode(value: impl AsRef<str>, base: Base) -> Result<String, DecodeError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Ok("".into());
        }
        let correct_base = Self::assert_encoding(value.as_bytes(), &base);
        if correct_base.is_err() {
            return correct_base.map(|_| String::new());
        }
        let indices = Self::into_table_idx(value, &base);
        if indices.is_err() {
            return indices.map(|_| "".into());
        }
        let indices = indices.unwrap();

        Self::into_string(match base {
            BASE64 => base64_decode(indices),
            BASE64URL => base64_url_decode(indices),
            BASE45 => base45_decode(indices),
            BASE32 => base32_decode(indices),
            BASE32HEX => base32_hex_decode(indices),
            BASE16 => base16_decode(indices),
        })
    }

    /// same as the decode function  but takes and returns raw Vec<u8>s instead of string types
    /// # Error
    /// * returns an error if the
    pub fn decode_bytes(value: Vec<u8>, base: Base) -> Result<Vec<u8>, DecodeError> {
        if value.is_empty() {
            return Ok(Vec::new());
        }
        let correct_base = Self::assert_encoding(&value, &base);
        if correct_base.is_err() {
            return correct_base.map(|_| Vec::new());
        }

        Ok(match base {
            BASE64 => base64_decode(value),
            BASE64URL => base64_url_decode(value),
            BASE45 => base45_decode(value),
            BASE32 => base32_decode(value),
            BASE32HEX => base32_hex_decode(value),
            BASE16 => base16_decode(value),
        })
    }

    /// decodes the given string
    /// takes encoded string
    /// returns `Ok(String)` value of decoded string at success
    /// or `Err(DecodeError)` in case of failure
    ///
    /// # Error
    /// returns an error when
    /// * `deduce_encoding` returns an error
    /// * the decode function returns an error that wasnt cought by `deduce_decoding`
    ///
    // NOTE was decode
    pub fn decode_deduce(value: impl AsRef<str>) -> Result<String, DecodeError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Ok("".into());
        }

        let base = match Self::deduce_encoding(value) {
            Err(e) => return Err(e),
            Ok(b) => b,
        };

        let correct_base = Self::assert_encoding(value.as_bytes(), &base);
        if correct_base.is_err() {
            return correct_base.map(|_| String::new());
        }

        let indices = Self::into_table_idx(value, &base);
        if indices.is_err() {
            return indices.map(|_| "".into());
        }
        let indices = indices.unwrap();

        Self::into_string(match base {
            BASE64 => base64_decode(indices),
            BASE64URL => base64_url_decode(indices),
            BASE45 => base45_decode(indices),
            BASE32 => base32_decode(indices),
            BASE32HEX => base32_hex_decode(indices),
            BASE16 => base16_decode(indices),
        })
    }

    // pub fn decode_loop(value: impl AsRef<str>) -> Result<String, DecodeError> {
    //     let value = value.as_ref();
    //     if value.is_empty() {
    //         return Ok("".into());
    //     }
    //
    //     let mut base = match Self::deduce_encoding(value) {
    //         Err(e) => return Err(e),
    //         Ok(b) => b,
    //     };
    //
    //     let mut correct_base = Self::assert_encoding(value.as_bytes(), &base);
    //     let mut exclude = vec![base];
    //     while correct_base.is_err() {
    //         if exclude.len() == 6 {
    //             break;
    //         }
    //         exclude.push(base);
    //         base = match Self::deduce_exclude(value, exclude.as_slice()) {
    //             Err(e) => return Err(e),
    //             Ok(b) => b,
    //         };
    //         correct_base = Self::assert_encoding(value.as_bytes(), &base);
    //     }
    //     if correct_base.is_err() {
    //         return correct_base.map(|_| String::new());
    //     }
    //
    //     let indices = Self::into_table_idx(value, &base);
    //     if indices.is_err() {
    //         return indices.map(|_| "".into());
    //     }
    //     let indices = indices.unwrap();
    //
    //     Self::into_string(match base {
    //         BASE64 => base64_decode(indices),
    //         BASE64URL => base64_url_decode(indices),
    //         BASE45 => base45_decode(indices),
    //         BASE32 => base32_decode(indices),
    //         BASE32HEX => base32_hex_decode(indices),
    //         BASE16 => base16_decode(indices),
    //     })
    // }

    /// asserts that the given vec of bytes is encoded with the given base
    pub fn assert_encoding(value: &[u8], base: &Base) -> Result<(), DecodeError> {
        let max = *value.into_iter().max().unwrap();
        let len = value.len();
        match base {
            Base::_64 | Base::_64URL => {
                if max < 64 && len % 4 == 0 {
                    Ok(())
                } else if len % 4 == 0 {
                    Err(DecodeError::TableIndexOverflow {
                        base: *base,
                        value: max,
                    })
                } else {
                    Err(DecodeError::BadLenForBase(len))
                }
            }
            Base::_45 => {
                if max < 45 && len % 3 != 1 {
                    Ok(())
                } else if len % 3 != 1 {
                    Err(DecodeError::TableIndexOverflow {
                        base: *base,
                        value: max,
                    })
                } else {
                    Err(DecodeError::BadLenForBase(len))
                }
            }
            Base::_32 | Base::_32HEX => {
                if max < 32 && len % 8 == 0 {
                    Ok(())
                } else if len % 8 == 0 {
                    Err(DecodeError::TableIndexOverflow {
                        base: *base,
                        value: max,
                    })
                } else {
                    Err(DecodeError::BadLenForBase(len))
                }
            }
            Base::_16 => {
                if max < 16 && len % 2 == 0 {
                    Ok(())
                } else if len % 2 == 0 {
                    Err(DecodeError::TableIndexOverflow {
                        base: *base,
                        value: max,
                    })
                } else {
                    Err(DecodeError::BadLenForBase(len))
                }
            }
        }
    }

    // deduction methods
    // 1 - length of value should be different between bases
    // 2 - existence of padding char '='
    // 3 - the char ranges, some chars are specific to some base(s) alphabets
    // 0 - if base encodings had patterns
    // then encoding can be further deduced through patterns
    /// Deduces the string encoding by process of elimination. Takes a base encoded string.
    ///
    /// # Error
    ///
    /// returns an `Ok(Base)` if no errors were found and a base was guessed safely, or an `Err(DecodeError)` if:
    ///
    /// * a base was deduced but string contains char(s) that don't belong to that base table
    /// * a base couldn't be deduced
    ///
    /// # Accuracy
    ///
    /// This function's deduction is not always correct for some bases,
    /// an example of this is the integrated decoder tests for base32 hex at `tests/base32_hex.rs`,
    /// test4 function panics when using `decode_deduce` instead of `decode` with a passed
    /// Base value
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

    // same as deduce_encoding but takes an additional exclude argument
    // that contains bases that are excluded from the deduction process
    pub fn deduce_exclude(value: &str, exclude: &[Base]) -> Result<Base, DecodeError> {
        let len = value.len();
        if value.contains(char::is_lowercase) && !exclude.are_excluded(&[BASE64, BASE64URL]) {
            if len % 4 != 0 {
                return Err(DecodeError::BadEncodedString);
            }

            // NOTE: even if the actual encoding is base64url
            // if no base64url specific chars are found
            // then it can be treated as normal base64
            return if value.contains(['_', '-']) && !exclude.is_excluded(&BASE64URL) {
                Ok(BASE64URL)
            } else if !exclude.is_excluded(&BASE64) {
                Ok(BASE64)
            } else {
                Err(DecodeError::EncodingBaseIsExcluded(BASE64))
            };
        } else if value
            .chars()
            .all(|c| ('0'..='9').contains(&c) || ('A'..='F').contains(&c))
            && !exclude.is_excluded(&BASE16)
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
        }) && !exclude.is_excluded(&BASE45)
        {
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
            && !exclude.is_excluded(&BASE32HEX)
        {
            return Ok(BASE32HEX);
        } else if !value.contains(['0', '1', '8', '9']) && !exclude.is_excluded(&BASE32)
        /* || value.contains(['W', 'X', 'Y', 'Z']) */ // this should be an and maybe condition
        {
            return Ok(BASE32);
        }

        Err(DecodeError::UnknownBaseEncodingIfAny)
    }
}

trait BaseExclusion {
    fn is_excluded(&self, base: &Base) -> bool;

    fn are_excluded(&self, bases: &[Base]) -> bool;
}

impl BaseExclusion for &[Base] {
    fn is_excluded(&self, base: &Base) -> bool {
        self.contains(base)
    }

    fn are_excluded(&self, bases: &[Base]) -> bool {
        bases.into_iter().all(|b| self.contains(b))
    }
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

    // BUG breaks deduce_encoding for base32hex and base16

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
