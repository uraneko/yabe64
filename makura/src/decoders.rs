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
#[derive(Debug, PartialEq, Clone)]
pub enum DecodeError {
    /// when decoding an encoded string that is supposed to be of base 16 or 45
    /// both of which can not contain padding '=' chars
    /// yet a padding char was found at the end of the encoded string
    NonPaddableEncoding(Base),
    /// results from trying togenerate a string from a Vec<u8> decoded bytes of an
    /// originally encoded string value
    ///
    /// this variant simply passes on the error value from the alloc::string::String::from_utf8
    /// String method
    Utf8Error(Utf8Error),
    /// string encoding is not any of the implemented base encodings
    /// i.e., it is not base 64, 64url, 45, 32, 32hex or 16 encoded
    ZeroValidEncodings,
    /// deducer has run all checks
    /// but more than one base encoding is valid
    TooManyValidEncodings { bases: Vec<Base> },
    /// occurs only on base64 and 32 encoding variants
    /// and only when there is padding on the encoded value
    /// indicates that the last char which should belong to a subset
    /// of the encoding table was out of that subset
    /// read DOCS section on src/decoders.rs mod deducer_pads
    InvalidLastCharForPadding { char: char, idx: u8, pads: u8 },
    /// padding value is invalid for passed encoding
    InvalidPadding { pads: u8, base: Base },
    /// encoded value chars and passed encoding base dont match
    // NOTE is this not still redundant with UnrecognizedCharForBase variant???
    InvalidChar { char: char, base: Base },
    /// encded value's len doesn't match with passed base
    InvalidLen { len: usize, base: Base },
}

// takes input value bytes
//
// returns last byte, len with pads, padding length
// TODO dont remove padding
// just change it to 0
fn input_meta(value: &mut &[u8]) -> (u8, usize, u8) {
    let len = value.len();
    let mut pads = 0u8;
    while value[len - pads as usize - 1] == '=' as u8 {
        pads += 1;
    }
    let last = value[len - pads as usize - 1];

    (last, len, pads)
}

// this only exists to match Encoder struct
// otherwise a free function works fine
pub struct Decoder;

impl Decoder {
    // turns back chars from the encoding table to their table index values
    fn into_table_idx(value: &[u8], base: &Base) -> Result<Vec<u8>, DecodeError> {
        // TODO convert paddings into necessary 0 bytes
        // in case they are not there
        let mut err: Option<DecodeError> = None;
        let val = value
            .into_iter()
            .map(|c| match *c as char {
                '=' => {
                    if base == &BASE16 || base == &BASE45 {
                        // this error is no longer reachable
                        Err(DecodeError::NonPaddableEncoding(*base))
                    } else {
                        Ok(0)
                    }
                }
                val => idx_from_char(val, base),
            })
            .take_while(|res| {
                if let Err(e) = res {
                    err = Some(e.clone());

                    false
                } else {
                    true
                }
            })
            .map(|res| res.unwrap())
            .collect::<Vec<u8>>();

        if let Some(e) = err {
            return Err(e);
        }

        Ok(val)
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
    pub fn decode<T: AsRef<[u8]>>(value: T, base: Base) -> Result<DecodeOutput, DecodeError> {
        let mut value = value.as_ref();
        if value.is_empty() {
            return Ok(Default::default());
        }

        let (last, len, pads) = input_meta(&mut value);

        let valid = base.is_valid_len(len);
        if valid.is_err() {
            return valid.map(|_| Default::default());
        }

        let valid = base.is_valid_padding(last, pads);
        if valid.is_err() {
            return valid.map(|_| Default::default());
        }

        let indices = Self::into_table_idx(value, &base);
        let indices = if indices.is_err() {
            return indices.map(|_| Default::default());
        } else {
            indices.unwrap()
        };

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

    pub fn decode_deduce<T: AsRef<[u8]> + core::fmt::Debug>(
        value: T,
    ) -> Result<DecodeOutput, DecodeError> {
        let base = Bases::deduce_default(&value);

        if base.is_err() {
            return base.map(|_| Default::default());
        }
        let base = base.unwrap();

        Self::decode(value, base)
    }

    /// same as using decode -> unwrap -. into_ascii
    pub fn decode_ascii<T: AsRef<[u8]>>(value: T, base: Base) -> Result<String, DecodeError> {
        let res = Self::decode(value, base);
        if res.is_err() {
            return res.map(|_| String::new());
        }

        Ok(res.unwrap().into_ascii())
    }

    /// same as using decode -> unwrap -. into_utf8
    pub fn decode_utf8<T: AsRef<[u8]>>(value: T, base: Base) -> Result<String, DecodeError> {
        let res = Self::decode(value, base);
        if res.is_err() {
            return res.map(|_| String::new());
        }

        res.unwrap().into_utf8()
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
            bases: BTreeSet::from_iter([BASE32, BASE32HEX, BASE16, BASE45, BASE64, BASE64URL]),
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

impl From<&mut Bases> for Vec<Base> {
    fn from(value: &mut Bases) -> Self {
        let mut val = core::mem::take(value);

        val.bases().into_iter().collect()
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
    ///
    /// this method always returns an error if there is more than 1 valid base
    /// it doesnt do estimations or guesses, only definitive answers
    pub fn deduce_encoding<T: AsRef<[u8]>>(&mut self, value: T) -> Result<Base, DecodeError> {
        let mut value = value.as_ref();
        if value.is_empty() {
            return Ok(BASE64);
        }

        let (last, len, pads) = input_meta(&mut value);

        *self = Self {
            bases: self
                .bases()
                .into_iter()
                .filter(|b| {
                    b.is_valid_len(len).is_ok()
                        && b.is_valid_padding(last, pads).is_ok()
                        && b.are_valid_chars(value).is_ok()
                })
                .collect(),
        };

        if self.is_empty() {
            return Err(DecodeError::ZeroValidEncodings);
        } else if self.len() == 1 {
            return self
                .bases_mut()
                .pop_first()
                .ok_or(unsafe { core::mem::zeroed() });
        }

        Err(DecodeError::TooManyValidEncodings { bases: self.into() })
    }

    /// same as deduce_encoding but this method will not error out
    /// when it finds more than 1 valid encoding
    ///
    /// instead, it will take the first encoding of self.bases as the correct encoding
    ///
    /// basically this considers the passed bases to be sorted
    /// and the least values (bases[0], base[1]...) as the most likely correct answer
    pub fn deduce_sorted<T: AsRef<[u8]>>(&mut self, value: T) -> Result<Base, DecodeError> {
        extern crate std;
        let mut value = value.as_ref();
        if value.is_empty() {
            return Ok(BASE64);
        }

        let (last, len, pads) = input_meta(&mut value);

        *self = Self {
            bases: self
                .bases()
                .into_iter()
                .inspect(|b| {
                    std::println!(
                        "{} -> chars={:?}|len={:?}|pads={:?}",
                        b,
                        b.are_valid_chars(value).is_ok(),
                        b.is_valid_len(len).is_ok(),
                        b.is_valid_padding(last, pads).is_ok()
                    )
                })
                .filter(|b| {
                    b.is_valid_len(len).is_ok()
                        && b.is_valid_padding(last, pads).is_ok()
                        && b.are_valid_chars(value).is_ok()
                })
                .collect(),
        };

        if self.is_empty() {
            return Err(DecodeError::ZeroValidEncodings);
        } else if self.len() == 1 {
            return self
                .bases_mut()
                .pop_first()
                .ok_or(unsafe { core::mem::zeroed() });
        } else if self.len() == 2 && self.contains(BASE64) && self.contains(BASE64URL) {
            // it is pretty common to have both base64 and 64url remain together
            // since '/' is very rare and '+' is a bit less rarer
            // so we prioritize base64
            return Ok(BASE64);
        }

        Ok(self.bases.pop_first().unwrap())
    }

    /// calls self's deduce_encoding on Self::default,
    /// which is all 6 known bases
    /// takes the input value to be analyzed
    pub fn deduce_default<T: AsRef<[u8]>>(value: T) -> Result<Base, DecodeError> {
        Self::default().deduce_sorted(value)
    }
}

mod deducer_chars {
    use super::*;

    const LWC: ops::RangeInclusive<u8> = 'a' as u8..='z' as u8;
    const UPC: ops::RangeInclusive<u8> = 'A' as u8..='Z' as u8;
    const NUM: ops::RangeInclusive<u8> = '0' as u8..='9' as u8;
    const HEX: ops::RangeInclusive<u8> = 'A' as u8..='F' as u8;
    const N32: ops::RangeInclusive<u8> = '2' as u8..='7' as u8;
    const PAD: u8 = '=' as u8;

    pub(super) fn chars_are_64(value: &[u8]) -> Result<(), DecodeError> {
        if let Some(e) = value
            .into_iter()
            .map(|c| {
                if UPC.contains(c)
                    || LWC.contains(c)
                    || NUM.contains(c)
                    || ['+' as u8, '/' as u8].contains(c)
                    || *c == PAD
                {
                    Ok(())
                } else {
                    Err(DecodeError::InvalidChar {
                        char: *c as char,
                        base: BASE64,
                    })
                }
            })
            .find(|res| res.is_err())
        {
            return e;
        }

        Ok(())
    }

    pub(super) fn chars_are_64url(value: &[u8]) -> Result<(), DecodeError> {
        if let Some(e) = value
            .into_iter()
            .map(|c| {
                if UPC.contains(c)
                    || LWC.contains(c)
                    || NUM.contains(c)
                    || ['-' as u8, '_' as u8].contains(c)
                    || *c == PAD
                {
                    Ok(())
                } else {
                    Err(DecodeError::InvalidChar {
                        char: *c as char,
                        base: BASE64URL,
                    })
                }
            })
            .find(|e| e.is_err())
        {
            return e;
        }

        Ok(())
    }

    pub(super) fn chars_are_45(value: &[u8]) -> Result<(), DecodeError> {
        if let Some(e) = value
            .into_iter()
            .map(|c| {
                if NUM.contains(c)
                    || UPC.contains(c)
                    || [
                        ' ' as u8, '$' as u8, '%' as u8, '*' as u8, '+' as u8, '-' as u8,
                        '.' as u8, '/' as u8, ':' as u8,
                    ]
                    .contains(c)
                {
                    Ok(())
                } else {
                    Err(DecodeError::InvalidChar {
                        char: *c as char,
                        base: BASE45,
                    })
                }
            })
            .find(|e| e.is_err())
        {
            return e;
        }

        Ok(())
    }

    pub(super) fn chars_are_32hex(value: &[u8]) -> Result<(), DecodeError> {
        if let Some(e) = value
            .into_iter()
            .map(|c| {
                if NUM.contains(c) || ('A' as u8..='V' as u8).contains(c) || *c == PAD {
                    Ok(())
                } else {
                    Err(DecodeError::InvalidChar {
                        char: *c as char,
                        base: BASE32HEX,
                    })
                }
            })
            .find(|e| e.is_err())
        {
            return e;
        }

        Ok(())
    }

    pub(super) fn chars_are_32(value: &[u8]) -> Result<(), DecodeError> {
        if let Some(e) = value
            .into_iter()
            .map(|c| {
                if UPC.contains(c) || N32.contains(c) || *c == PAD {
                    Ok(())
                } else {
                    Err(DecodeError::InvalidChar {
                        char: *c as char,
                        base: BASE32,
                    })
                }
            })
            .find(|e| e.is_err())
        {
            return e;
        }

        Ok(())
    }

    pub(super) fn chars_are_16(value: &[u8]) -> Result<(), DecodeError> {
        if let Some(e) = value
            .into_iter()
            .map(|c| {
                if NUM.contains(c) || HEX.contains(c) {
                    Ok(())
                } else {
                    Err(DecodeError::InvalidChar {
                        char: *c as char,
                        base: BASE16,
                    })
                }
            })
            .find(|e| e.is_err())
        {
            return e;
        }

        Ok(())
    }

    #[cfg(test)]
    mod test_chars {
        use super::*;

        #[test]
        fn test0_64url() {
            let output = "pl-";

            assert_eq!(chars_are_64url(output.as_bytes()), Ok(()));
        }

        #[test]
        fn test1_64url() {
            let output = "sqw_";

            assert_eq!(chars_are_64url(output.as_bytes()), Ok(()));
        }

        #[test]
        fn test0_64() {
            let output = "sqw+";

            assert_eq!(chars_are_64(output.as_bytes()), Ok(()));
        }

        #[test]
        fn test1_64() {
            let output = "sqw/";

            assert_eq!(chars_are_64(output.as_bytes()), Ok(()));
        }

        #[test]
        fn test2_64() {
            let output = "12e2e23cSIJOA";

            assert_eq!(chars_are_64(output.as_bytes()), Ok(()));
        }

        #[test]
        fn test0_45() {
            let output = "CSAL $%*+-./:";

            assert_eq!(chars_are_45(output.as_bytes()), Ok(()));
        }

        #[test]
        fn test_32hex() {
            let output = "49312ASC";

            assert_eq!(chars_are_32hex(output.as_bytes()), Ok(()));
        }

        #[test]
        #[should_panic]
        fn fail_32hex() {
            let output = "697JHGX";

            assert_eq!(chars_are_32hex(output.as_bytes()), Ok(()));
        }

        #[test]
        fn test_32() {
            let output = "AZSX5672";

            assert_eq!(chars_are_32(output.as_bytes()), Ok(()));
        }

        #[test]
        #[should_panic]
        fn fail_32() {
            let output = "1SA";

            assert_eq!(chars_are_32(output.as_bytes()), Ok(()));
        }

        #[test]
        fn test_16() {
            let output = "6587AF";

            assert_eq!(chars_are_16(output.as_bytes()), Ok(()));
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
    use super::DecodeError;
    use super::{BASE32, BASE32HEX, BASE64, BASE64URL};
    use crate::idx_from_char;

    // NOTE pad = 0 and pad = invalid value are both to be handled by the
    // only function calling these fns
    //
    // this is fine since these are internal fns, not part of the public api
    // otherwise, checking irrelevant (0, invalid) values at every is_valid_x_padding fn is a pain
    //
    // this fn expects pads to be a valid base64 padding value
    pub(super) fn is_valid_64_padding(last_byte: u8, pads: u8) -> Result<(), DecodeError> {
        let char = last_byte as char;
        let last_byte = idx_from_char(char, &BASE64);
        if last_byte.is_err() {
            return last_byte.map(|_| ());
        }
        let last_byte = last_byte.unwrap();

        match pads {
            1 if last_byte % 4 == 0 => Ok(()),
            2 if last_byte % 16 == 0 => Ok(()),
            1 | 2 => Err(DecodeError::InvalidLastCharForPadding {
                char,
                idx: last_byte,
                pads,
            }),
            _ => unreachable!("both 0 and invalid values were checked before getting here"),
        }
    }

    pub(super) fn is_valid_64url_padding(last_byte: u8, pads: u8) -> Result<(), DecodeError> {
        let char = last_byte as char;
        let last_byte = idx_from_char(char, &BASE64URL);
        if last_byte.is_err() {
            return last_byte.map(|_| ());
        }
        let last_byte = last_byte.unwrap();

        match pads {
            1 if last_byte % 4 == 0 => Ok(()),
            2 if last_byte % 16 == 0 => Ok(()),
            1 | 2 => Err(DecodeError::InvalidLastCharForPadding {
                char,
                idx: last_byte,
                pads,
            }),
            _ => unreachable!("both 0 and invalid values were checked before getting here"),
        }
    }

    pub(super) fn is_valid_32_padding(last_byte: u8, pads: u8) -> Result<(), DecodeError> {
        let char = last_byte as char;
        let last_byte = idx_from_char(char, &BASE32);
        if last_byte.is_err() {
            return last_byte.map(|_| ());
        }
        let last_byte = last_byte.unwrap();

        match pads {
            1 if last_byte % 8 == 0 => Ok(()),
            3 if last_byte % 2 == 0 => Ok(()),
            4 if last_byte % 16 == 0 => Ok(()),
            6 if last_byte % 4 == 0 => Ok(()),
            1 | 3 | 4 | 6 => Err(DecodeError::InvalidLastCharForPadding {
                char,
                idx: last_byte,
                pads,
            }),
            _ => unreachable!("both 0 and invalid values were checked before getting here"),
        }
    }

    pub(super) fn is_valid_32hex_padding(last_byte: u8, pads: u8) -> Result<(), DecodeError> {
        let char = last_byte as char;
        let last_byte = idx_from_char(char, &BASE32HEX);
        if last_byte.is_err() {
            return last_byte.map(|_| ());
        }
        let last_byte = last_byte.unwrap();

        match pads {
            1 if last_byte % 8 == 0 => Ok(()),
            3 if last_byte % 2 == 0 => Ok(()),
            4 if last_byte % 16 == 0 => Ok(()),
            6 if last_byte % 4 == 0 => Ok(()),
            1 | 3 | 4 | 6 => Err(DecodeError::InvalidLastCharForPadding {
                char,
                idx: last_byte,
                pads,
            }),
            _ => unreachable!("both 0 and invalid values were checked before getting here"),
        }
    }
}

// TODO fn a(s: impl Trai) s here is of anonymous type, which is not the same as fn a<T: Trai>(s: T)
// use the latter, type generics instead of anonymous types
// using anon types:
// * all types are agnostic to each other
// * cant use turbofish to specify generic type on fn call
// NOTE impl Trait is useful in return types tho,
// since it specifies that return type is anonymous
// contrary to type generics, which are considered to be concrete types assuming i understood
// correctly

// the len checks should go first  <- least costly
// then the pad checks            <- in between
// then finally the chars checks <- costliest
mod deducer_len {
    use super::DecodeError;
    use super::{BASE16, BASE32, BASE45, BASE64};

    pub(super) fn is_valid_64_len(len: usize) -> Result<(), DecodeError> {
        if len % 4 == 0 {
            Ok(())
        } else {
            Err(DecodeError::InvalidLen { len, base: BASE64 })
        }
    }

    pub(super) fn is_valid_32_len(len: usize) -> Result<(), DecodeError> {
        if len % 8 == 0 {
            Ok(())
        } else {
            Err(DecodeError::InvalidLen { len, base: BASE32 })
        }
    }

    pub(super) fn is_valid_16_len(len: usize) -> Result<(), DecodeError> {
        if len % 2 == 0 {
            Ok(())
        } else {
            Err(DecodeError::InvalidLen { len, base: BASE16 })
        }
    }

    pub(super) fn is_valid_45_len(len: usize) -> Result<(), DecodeError> {
        if len % 3 != 1 {
            Ok(())
        } else {
            Err(DecodeError::InvalidLen { len, base: BASE45 })
        }
    }
}

impl Base {
    pub fn is_valid_padding(&self, last_byte: u8, pads: u8) -> Result<(), DecodeError> {
        use deducer_pads::*;

        if pads == 0 {
            return Ok(());
        }

        match pads {
            1 if BASE64 == *self => is_valid_64_padding(last_byte, pads),
            1 if BASE64URL == *self => is_valid_64url_padding(last_byte, pads),
            1 if BASE32 == *self => is_valid_32_padding(last_byte, pads),
            1 if BASE32HEX == *self => is_valid_32hex_padding(last_byte, pads),
            2 if BASE64 == *self => is_valid_64_padding(last_byte, pads),
            2 if BASE64URL == *self => is_valid_64url_padding(last_byte, pads),
            1 | 2 => Err(DecodeError::NonPaddableEncoding(*self)),
            3 | 4 | 6 if BASE32 == *self => is_valid_32_padding(last_byte, pads),
            3 | 4 | 6 if BASE32HEX == *self => is_valid_32hex_padding(last_byte, pads),
            _ if BASE45 == *self || BASE16 == *self => Err(DecodeError::NonPaddableEncoding(*self)),
            _ => Err(DecodeError::InvalidPadding { base: *self, pads }),
        }
    }

    // NOTE this doesnt differenciate between hex and url variants
    pub fn is_valid_len(&self, len: usize) -> Result<(), DecodeError> {
        use deducer_len::*;

        match *self {
            BASE64 | BASE64URL => is_valid_64_len(len),
            BASE45 => is_valid_45_len(len),
            BASE32 | BASE32HEX => is_valid_32_len(len),
            BASE16 => is_valid_16_len(len),
        }
    }

    /// checks whether all bytes of input
    /// match self's value
    pub fn are_valid_chars(&self, input: &[u8]) -> Result<(), DecodeError> {
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
}

#[cfg(feature = "nightly")]
// TODO: fix this; use test/bench api
// this module benchmarks different versions of the deduce_encoding Decoder function
mod bench_deduce_encoding {
    extern crate test;
    use test::Bencher;

    const DATA: &str = "io8yyioljb";

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
                super::Bases::deduce_default(&e).unwrap();
            })
        });
    }
}

/// this module tests that the decoding errors happen as intended when they are supposed to
#[cfg(test)]
mod test_errors {
    use super::vec;
    use super::{BASE16, BASE32, BASE32HEX, BASE64, BASE64URL};
    use super::{DecodeError, Decoder};

    #[test]
    fn zero_valid_encodings() {
        let input = "@";
        let Err(e) = super::Bases::default().deduce_encoding(input) else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };

        assert_eq!(e, DecodeError::ZeroValidEncodings);
    }

    #[test]
    // BUG this panicked cause of decoders/base32.rs:37:21:
    // index out of bounds: the len is 4 but the index is 4
    // TODO account for zeroes when less than 1 chunk exists in decoded input
    fn too_many_valid_encodings() {
        let output = "AA==";
        let Err(e) = super::Bases::default().deduce_encoding(output) else {
            unreachable!("this should have been an error");
        };

        assert_eq!(
            e,
            DecodeError::TooManyValidEncodings {
                bases: vec![BASE64, BASE64URL]
            }
        );
    }

    // NOTE this error variant can't be reachaed in current impl
    // since it is superseeded by InvalidPadding variant
    #[test]
    fn non_paddable_encoding() {
        let input = "09==";
        let Err(e) = Decoder::into_table_idx(input.as_bytes(), &BASE16) else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };

        assert_eq!(e, DecodeError::NonPaddableEncoding(BASE16));
    }

    #[test]
    fn invalid_len() {
        let input = "123";
        let Err(e) = Decoder::decode(input, BASE64) else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };

        assert_eq!(
            e,
            DecodeError::InvalidLen {
                len: 3,
                base: BASE64
            }
        );
    }

    #[test]
    fn invalid_padding() {
        let output = "AAA=====";
        let Err(e) = Decoder::decode(output, BASE32) else {
            unreachable!("this should have been an error");
        };

        assert_eq!(
            e,
            DecodeError::InvalidPadding {
                base: BASE32,
                pads: 5
            }
        );
    }

    #[test]
    fn invalid_char() {
        // let input = "VT09PQ==";
        let input = "VT";
        let Err(e) = Decoder::decode(input, BASE16) else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };

        assert_eq!(
            e,
            DecodeError::InvalidChar {
                char: 'V',
                base: BASE16
            }
        );
    }

    #[test]
    fn invalid_last_char_for_padding() {
        let output = "AAAD====";
        let Err(e) = Decoder::decode(output, BASE32) else {
            unreachable!("this should have been an error");
        };

        assert_eq!(
            e,
            DecodeError::InvalidLastCharForPadding {
                char: 'D',
                pads: 4,
                idx: 3
            }
        );
    }

    #[test]
    fn utf8_error() {
        let input = [65, 66];

        let Err(DecodeError::Utf8Error(e)) = Decoder::decode_utf8(input, BASE16) else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };
    }
}
