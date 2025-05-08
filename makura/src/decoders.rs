#![cfg(feature = "decoding")]
use crate::makura_alloc::{BTreeSet, Cow, String, Vec, vec};
use crate::makura_core::Utf8Error;
use crate::makura_core::ops;

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

#[derive(Debug, Clone, Default)]
pub struct DecodeOutput {
    value: Vec<u8>,
}

impl DecodeOutput {
    /// turns the decoded bytes into an ascii string
    pub fn into_ascii(self) -> String {
        self.value
            .into_iter()
            .map(|c| c as char)
            .collect::<String>()
    }

    /// turns the decoded bytes into an utf8 string
    pub fn into_utf8(self) -> Result<String, DecodeError> {
        let res = String::from_utf8(self.value);
        if res.is_ok() {
            // NOTE quick, call the unsafe police
            res.map_err(|_| unsafe { core::mem::zeroed::<_>() })
        } else {
            res.map_err(|e| DecodeError::Utf8Error(e.utf8_error()))
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.value
    }

    pub fn as_ascii(&self) -> Cow<'_, str> {
        self.value.iter().map(|c| *c as char).collect()
    }

    pub fn as_utf8_lossy(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.value)
    }

    pub fn as_utf8(&self) -> Result<&str, DecodeError> {
        core::str::from_utf8(self.value.as_slice()).map_err(|e| DecodeError::Utf8Error(e))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.value.as_slice()
    }
}

impl From<Vec<u8>> for DecodeOutput {
    fn from(value: Vec<u8>) -> Self {
        Self { value }
    }
}

struct DecodeOutputRef<'a> {
    value: &'a [u8],
}

impl<'a> From<&'a [u8]> for DecodeOutputRef<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self { value }
    }
}

/// errors that can occur during the decoding process of some base encoded input value
#[derive(Debug, PartialEq)]
pub enum DecodeError {
    /// string was deduced to be base<x> encoded but contains char(s) that
    /// don't belong to base<x>'s encoding table
    // #[deprecated]
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
    BaseEncodingNotFoundForInput,
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
    // #[deprecated]
    TableIndexOverflow { base: Base, value: u8 },
    /// when decoding an encoded string that is supposed to be of base 16 or 45
    /// both of which can not contain padding '=' chars
    /// yet a padding char was found at the end of the encoded string
    NotAPaddableBaseEncoding(Base),
    /// results from trying togenerate a string from a Vec<u8> decoded bytes of an
    /// originally encoded string value
    ///
    /// this variant simply passes on the error value from the alloc::string::String::from_utf8
    /// String method
    Utf8Error(Utf8Error),
    /// can only be reached from the deduce_exclude Decoder function
    /// signifies that the correct base was deduced but it has been excluded from the deduction
    /// the deduction process exits with this error value since further deduction is useless
    EncodingBaseIsExcluded(Base),
}

// this only exists to match Encoder struct
// otherwise a free function works fine
pub struct Decoder;

impl Decoder {
    // turns back chars from the encoding table to their table index values
    fn into_table_idx(value: &[u8], base: &Base) -> Result<Vec<u8>, DecodeError> {
        extern crate std;
        // no need for chars count, len is sufficient since all chars are ascii (1 byte)
        // WARN they are not all ascii, baseless assumption
        // but i cant recall what the line above is talking about
        let mut val = value.into_iter().map(|c| match *c as char {
            '=' => {
                if base == &BASE16 || base == &BASE45 {
                    Err(DecodeError::NotAPaddableBaseEncoding(*base))
                } else {
                    Ok(0)
                }
            }
            val => idx_from_char(val, base),
        });
        if val.clone().any(|i| i.is_err()) {
            std::println!("0");
            return val.find(|i| i.is_err()).unwrap().map(|_| vec![]);
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
    pub fn decode(value: impl AsRef<[u8]>, base: Base) -> Result<DecodeOutput, DecodeError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Ok(Default::default());
        }

        let valid_len = base.validate_len(value);
        if valid_len.is_err() {
            return valid_len.map(|_| DecodeOutput::default());
        }

        let indices = Self::into_table_idx(value, &base);
        if indices.is_err() {
            return indices.map(|_| DecodeOutput::default());
        }
        let indices = indices.unwrap();

        Ok(match base {
            BASE64 => base64_decode(indices),
            BASE64URL => base64_url_decode(indices),
            BASE45 => base45_decode(indices),
            BASE32 => base32_decode(indices),
            BASE32HEX => base32_hex_decode(indices),
            BASE16 => base16_decode(indices),
        }
        .into())
    }

    /// same as the decode function  but takes and returns raw Vec<u8>s instead of string types
    /// # Error
    /// * returns an error if the
    #[deprecated(
        note = "use decode instead, since decode can now take &[u8] and all decode fns now return Vec<u8>"
    )]
    pub fn decode_bytes(value: Vec<u8>, base: Base) -> Result<Vec<u8>, DecodeError> {
        if value.is_empty() {
            return Ok(Vec::new());
        }
        // it's fine here since the bytes are user provided
        // so they must be validated
        let correct_base = base.assert_encoding(&value);
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
    pub fn decode_deduce(value: impl AsRef<[u8]>) -> Result<DecodeOutput, DecodeError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Ok(Default::default());
        }

        let base = match Bases::default().deduce_encoding(&value) {
            Err(e) => return Err(e),
            Ok(b) => b,
        };

        let valid_len = base.validate_len(&value);
        if valid_len.is_err() {
            return valid_len.map(|_| unsafe { core::mem::zeroed::<_>() });
        }

        let indices = Self::into_table_idx(value, &base);
        if indices.is_err() {
            return indices.map(|_| unsafe { core::mem::zeroed::<_>() });
        }
        let indices = indices.unwrap();

        Ok(match base {
            BASE64 => base64_decode(indices),
            BASE64URL => base64_url_decode(indices),
            BASE45 => base45_decode(indices),
            BASE32 => base32_decode(indices),
            BASE32HEX => base32_hex_decode(indices),
            BASE16 => base16_decode(indices),
        }
        .into())
    }

    #[deprecated(note = "use BaseSlice::default().deduce_encoding() instead")]
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
        } else if value
            .chars()
            .all(|c| ('A'..='Z').contains(&c) || ('2'..='7').contains(&c))
        /* || value.contains(['W', 'X', 'Y', 'Z']) */ // this should be an and maybe condition
        {
            return Ok(BASE32);
        }

        Err(DecodeError::BaseEncodingNotFoundForInput)
    }
}

/// a set of bases (Base)
///
/// uses a BTreeSet for its inner value
#[derive(Debug, Clone)]
pub struct Bases {
    bases: BTreeSet<Base>,
}

impl Default for Bases {
    fn default() -> Self {
        Self {
            bases: BTreeSet::from_iter([BASE64, BASE64URL, BASE45, BASE32, BASE32HEX, BASE16]),
        }
    }
}

impl From<&[Base]> for Bases {
    fn from(value: &[Base]) -> Self {
        Self {
            bases: value.into_iter().map(|b| *b).collect(),
        }
    }
}

impl Bases {
    /// calls self's deduce_encoding on Self::default,
    /// which is all 6 known bases
    /// takes the input value to be analyzed
    pub fn deduce_default<T: AsRef<[u8]>>(value: T) -> Result<Base, DecodeError> {
        Self::default().deduce_encoding(value)
    }
}

impl Bases {
    /// returns a new Bases with an empty BTreeSet
    pub fn new() -> Self {
        Self {
            bases: BTreeSet::new(),
        }
    }

    /// delegation of BTreeSet's contains method
    pub fn contains(&self, base: Base) -> bool {
        self.bases.contains(&base)
    }

    /// delegation of BTreeSet's insert method
    pub fn insert(&mut self, base: Base) -> bool {
        self.bases.insert(base)
    }

    /// delegation of BTreeSet's remove method
    pub fn remove(&mut self, base: Base) -> bool {
        self.bases.remove(&base)
    }

    /// delegation of BTreeSet's clear method
    pub fn clear(&mut self) {
        self.bases.clear()
    }

    /// delegation of BTreeSet's is_empty method
    pub fn is_empty(&self) -> bool {
        self.bases.is_empty()
    }

    /// delegation of BTreeSet's len method
    pub fn len(&self) -> usize {
        self.bases.len()
    }

    /// returns the owned inner value,
    ///  doesnt consume self
    ///
    /// changes self's inner value to BTreeSet::default() | new()
    pub fn bases(&mut self) -> BTreeSet<Base> {
        core::mem::take(&mut self.bases)
    }

    /// returns an immutable reference to the inner BTreeSet value
    pub fn bases_ref(&self) -> &BTreeSet<Base> {
        &self.bases
    }

    /// returns a mutable reference to the inner BTreeSet value
    pub fn bases_mut(&mut self) -> &mut BTreeSet<Base> {
        &mut self.bases
    }

    // DOCS encloding validation
    // * len matches check
    // * all chars match check
    // * the existence and number of padding chars '='
    //
    /// Deduces the string encoding by process of elimination. Takes a base encoded string.
    /// This method modifies self's inner value in place
    ///
    /// for a version that doesn't modify self (clones the inner value),
    /// use deduce_cloned
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
    pub fn deduce_encoding<T: AsRef<[u8]>>(&mut self, value: T) -> Result<Base, DecodeError> {
        extern crate std;

        let mut value = value.as_ref();
        let len = value.len();
        let mut pads = 0;
        while value.ends_with(&['=' as u8]) {
            pads += 1;
            value = value.strip_suffix(&[('=' as u8)]).unwrap();
        }
        std::println!("pads = {}", pads);

        match pads {
            1 | 2 => {
                self.remove(BASE45);
                self.remove(BASE16);
            }
            3 | 4 | 6 => {
                self.remove(BASE45);
                self.remove(BASE16);
                self.remove(BASE64);
                self.remove(BASE64URL);
            }
            0 => (),
            pads => unreachable!(
                "{} is not a valid padding len for base64 or 32 encodings",
                pads
            ),
        }
        std::println!("bases = {:?}", self.bases);

        *self = Self {
            bases: self
                .bases()
                .into_iter()
                .filter(|b| b.chars_match(value))
                .collect(),
        };
        std::println!("bases = {:?}", self.bases);

        if self.is_empty() {
            return Err(DecodeError::BaseEncodingNotFoundForInput);
        } else if self.len() == 1 {
            return self
                .bases_mut()
                .pop_first()
                .ok_or(unsafe { core::mem::zeroed() });
        }
        std::println!("{}", len);

        match len {
            len if self.contains(BASE16) && len % 2 == 0 => Ok(BASE16),
            len if self.contains(BASE32HEX) && len % 8 == 0 => Ok(BASE32HEX),
            len if self.contains(BASE32) && len % 8 == 0 => Ok(BASE32),
            len if self.contains(BASE45) && len % 3 != 1 => Ok(BASE45),
            len if self.contains(BASE64URL) && len % 4 == 0 => Ok(BASE64URL),
            len if self.contains(BASE64) && len % 4 == 0 => Ok(BASE64),
            _ => Err(DecodeError::BaseEncodingNotFoundForInput),
        }
    }
}

mod deducer_chars {
    use super::*;

    pub const LWC: ops::RangeInclusive<u8> = 'a' as u8..='z' as u8;
    pub const UPC: ops::RangeInclusive<u8> = 'A' as u8..='Z' as u8;
    pub const NUM: ops::RangeInclusive<u8> = '0' as u8..='9' as u8;
    pub const HEX: ops::RangeInclusive<u8> = 'A' as u8..='F' as u8;
    pub const N32: ops::RangeInclusive<u8> = '2' as u8..='7' as u8;

    pub(super) fn chars_are_64(value: &[u8]) -> bool {
        extern crate std;
        value
            .into_iter()
            .inspect(|c| std::print!("{}, ", c))
            .all(|c| {
                UPC.contains(c)
                    || LWC.contains(c)
                    || NUM.contains(c)
                    || ['+' as u8, '/' as u8].contains(c)
            })
    }

    pub(super) fn chars_are_64url(value: &[u8]) -> bool {
        value.into_iter().all(|c| {
            UPC.contains(c)
                || LWC.contains(c)
                || NUM.contains(c)
                || ['-' as u8, '_' as u8].contains(c)
        })
    }

    pub(super) fn chars_are_45(value: &[u8]) -> bool {
        value.into_iter().all(|c| {
            NUM.contains(c)
                || UPC.contains(c)
                || [
                    ' ' as u8, '$' as u8, '%' as u8, '*' as u8, '+' as u8, '-' as u8, '.' as u8,
                    '/' as u8, ':' as u8,
                ]
                .contains(c)
        })
    }

    pub(super) fn chars_are_32hex(value: &[u8]) -> bool {
        value
            .into_iter()
            .all(|c| NUM.contains(c) || ('A' as u8..='V' as u8).contains(c))
    }

    pub(super) fn chars_are_32(value: &[u8]) -> bool {
        value
            .into_iter()
            .all(|c| UPC.contains(c) || N32.contains(c))
    }

    pub(super) fn chars_are_16(value: &[u8]) -> bool {
        value
            .into_iter()
            .all(|c| NUM.contains(c) || HEX.contains(c))
    }

    #[cfg(test)]
    mod test_chars {
        use super::*;

        #[test]
        fn test0_64url() {
            let output = "pl-";

            assert_eq!(chars_are_64url(output.as_bytes()), true);
        }

        #[test]
        fn test1_64url() {
            let output = "sqw_";

            assert_eq!(chars_are_64url(output.as_bytes()), true);
        }

        #[test]
        fn test0_64() {
            let output = "sqw+";

            assert_eq!(chars_are_64(output.as_bytes()), true);
        }

        #[test]
        fn test1_64() {
            let output = "sqw/";

            assert_eq!(chars_are_64(output.as_bytes()), true);
        }

        #[test]
        fn test2_64() {
            let output = "12e2e23cSIJOA";

            assert_eq!(chars_are_64(output.as_bytes()), true);
        }

        #[test]
        fn test0_45() {
            let output = "CSAL $%*+-./:";

            assert_eq!(chars_are_45(output.as_bytes()), true);
        }

        #[test]
        fn test_32hex() {
            let output = "49312ASC";

            assert_eq!(chars_are_32hex(output.as_bytes()), true);
        }

        #[test]
        #[should_panic]
        fn fail_32hex() {
            let output = "697JHGX";

            assert_eq!(chars_are_32hex(output.as_bytes()), true);
        }

        #[test]
        fn test_32() {
            let output = "AZSX5672";

            assert_eq!(chars_are_32(output.as_bytes()), true);
        }

        #[test]
        #[should_panic]
        fn fail_32() {
            let output = "1SA";

            assert_eq!(chars_are_32(output.as_bytes()), true);
        }

        #[test]
        fn test_16() {
            let output = "6587AF";

            assert_eq!(chars_are_16(output.as_bytes()), true);
        }
    }
}

// DOCS:
// technically we can not get a B, C or D at the end of a byte
// we can only get such values at the beginning of a byte
// let me elaborate
// for an input value = 0b0000_0001
// the output value will be = 0b00000, 0b001
// the second bit will then be padded by 2 negative bits 00
// rendering an output of: 0b00000, 0b00100 -> AE
// so to say,the smallest positive bit value of 1 can never be generated at the end of a byte
// this is the case for 1,2 and 3 they can only be at the start of a byte like so: 0b0000_100,
// taking the first 5 bits; the first encoded value will be a B
// consequently, we can never get any values in between 0 and 4 in a base32 encoding from the first
// u5 byte,
// that is, if we have a 2 chars input value starting with some_char
// the second char can only be
// the 0th, 4th, 8th, 12th, 16th... char in the base32 encoding table
// this is because we always pad the second value by 2 zeroes
// and we do that, the smallest value of the second u5 byte is 0 followed by 100 which is four
// all possible values of the second byte will have to be multiples of 4
//
// in conclusion: for every input value I which is base32 encoded, assuming that I is padded
// such that NP is the number of padding chars and CL is the length of the chunk containing the last bytes:
// -> NP depends upon CL, e.g., if CL = 1
// => 1 byte of 1st 5 bits and 2nd byte of last 3 bits (padded by 00) = 2 bytes in chunk
// =>  NP = 8 - 2 = 6
// there can only be the following cases for the smallest non zero value of the last byte(u5) LB:
// * if CL = 1 && NP = 6 => LB = 001
// -> padded by least bits 00 => LB = 00100, is always a multiple of 4
//
// * if CL = 2 && NP = 4 => LB = 1
// -> padded by least bits 0000 => LB = 10000, is always a multiple of 16
//
// * if CL = 3 && NP = 3 => LB = 0001
// -> padded by least bit 0 => LB = 00010, is always a multiple of 2
//
// * if CL = 4 && NP = 1 => LB = 01
// -> padded by least bits 000 => LB = 01000, is always a multiple of 8
//
// * if CL = 5 && NP = 0 => the last value can be any value in the base32 encoding table
//
//
// likewise for base64, there can only be the following padded input cases:
// * if CL = 1 && NP = 2 => LB = 01
// -> padded by least bits 0000 => LB = 010_000, is always a multiple of 16
//
// * if CL = 2 && NP = 1 => LB = 0001;
// -> padded by least bits 00 => LB = 000_100, is always a multiple of 4
// ^_ since [16 = 6 * 2 + 4] we already have 3 values, but add
// a padding char to indicate that the last byte value was padded by least bits 00
//
// * if CL = 3 && NP = 0 => the last value can be any value in the base64 encoding table
mod deducer_pads {
    fn is_valid_64_padding() {}

    fn is_valid_32_padding() {}
}

mod deducer_len {}

impl Base {
    /// checks whether all bytes of input
    /// match self's value
    pub fn chars_match(&self, input: &[u8]) -> bool {
        use deducer_chars::*;
        match *self {
            // FIXME it's quite redundant to do both a 64 and a 64 url checks
            BASE64 => chars_are_64(input),
            BASE64URL => chars_are_64url(input),
            BASE45 => chars_are_45(input),
            BASE32 => chars_are_32(input),
            BASE32HEX => chars_are_32hex(input),
            BASE16 => chars_are_16(input),
        }
    }

    /// asserts that the given vec of bytes is encoded with the given base
    // NOTE using this in the decode functions is redundant
    // since any bad chars are cought by the into_table_idx function
    pub fn assert_encoding(&self, value: &[u8]) -> Result<(), DecodeError> {
        let max = *value.into_iter().max().unwrap();
        let len = value.len();
        match self {
            Base::_64 | Base::_64URL => {
                if max < 64 && len % 4 == 0 {
                    Ok(())
                } else if len % 4 == 0 {
                    Err(DecodeError::TableIndexOverflow {
                        base: *self,
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
                        base: *self,
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
                        base: *self,
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
                        base: *self,
                        value: max,
                    })
                } else {
                    Err(DecodeError::BadLenForBase(len))
                }
            }
        }
    }

    pub fn validate_len(&self, value: &[u8]) -> Result<(), DecodeError> {
        let len = value.len();
        match *self {
            BASE64 | BASE64URL if len % 4 != 0 => Err(DecodeError::BadLenForBase(len)),
            BASE45 if len % 3 == 1 => Err(DecodeError::BadLenForBase(len)),
            BASE32 | BASE32HEX if len % 8 != 0 => Err(DecodeError::BadLenForBase(len)),
            BASE16 if len % 2 != 0 => Err(DecodeError::BadLenForBase(len)),
            // the opposite of all the above arms
            _ => Ok(()),
        }
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

/// this module tests that the decoding errors happen as intended when they are supposed to
#[cfg(test)]
mod test_errors {
    use core::mem::discriminant;

    use super::{BASE16, BASE32, BASE32HEX, BASE64};
    use super::{DecodeError, Decoder};

    // BUG decoding "1239" from base

    // #[test]
    // fn err_bad_encoding() {
    //     let input = "foobar";
    //     let Err(e) = Decoder::decode(input, BASE64) else {
    //         unreachable!("input string is not proper base64 encoded, so how did it pass")
    //     };
    //
    //     assert_eq!(e, DecodeError::BadEncodedString);
    // }
    // TODO remove BadEncodedString variant -> it should be never reached
    // anywhere in the current design, deprecated by the newer variants

    #[test]
    fn err_bad_len_for_base() {
        let input = "123";
        let Err(e) = Decoder::decode(input, BASE64) else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };

        assert_eq!(e, DecodeError::BadLenForBase(3));
    }

    #[test]
    fn err_undetected_base_encoding() {
        let input = "@";
        let Err(e) = super::Bases::default().deduce_encoding(input) else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };

        assert_eq!(e, DecodeError::BaseEncodingNotFoundForInput);
    }

    #[test]
    fn err_unrecognized_char_for_base() {
        let input = "VT09PQ==";
        let Err(e) = Decoder::decode(input, BASE16) else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };

        assert_eq!(
            e,
            DecodeError::UnrecognizedCharForBase {
                ch: 'V',
                base: BASE16
            }
        );
    }

    // TODO remove this variant
    // UnrecognizedCharForBase already catches the same error
    // this variant is redundant
    // #[test]
    // fn err_table_index_overflow() {
    //     let input = "";
    //     let Err(e) = Decoder::decode(input, BASE64) else {
    //         unreachable!("input string is not proper base64 encoded, so how did it pass")
    //     };
    //
    //     assert_eq!(
    //         e,
    //         DecodeError::TableIndexOverflow {
    //             base: BASE64,
    //             value: 120
    //         },
    //     );
    // }

    #[test]
    fn err_not_a_paddable_base_encoding() {
        let input = "09==";
        let Err(e) = Decoder::decode(input, BASE16) else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };

        assert_eq!(e, DecodeError::NotAPaddableBaseEncoding(BASE16));
    }

    #[test]
    fn err_utf8_error() {
        let input = "1239";

        let Err(DecodeError::Utf8Error(e)) = Decoder::decode(input, BASE64).unwrap().into_utf8()
        else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };

        assert!(core::any::type_name_of_val(&e).ends_with("Utf8Error"));

        assert_eq!(e.error_len(), Some(1));
    }

    #[test]
    fn err_encoding_base_is_excluded() {
        // let input = "ABC=====";
        // panic!("{:?}", Decoder::deduce_exclude(input, &[BASE32, BASE32HEX]));
        // let Err(e) = Decoder::deduce_exclude(input, &[BASE32]) else {
        //     unreachable!("input string is not proper base64 encoded, so how did it pass")
        // };
        //
        // assert_eq!(
        //     discriminant(&e),
        //     discriminant(&DecodeError::BadEncodedString)
        // );
    }
}
