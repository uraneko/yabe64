use super::char_checks::*;
use super::{Base, idx_from_char};

pub mod base16;
pub mod base32;
pub mod base45;
pub mod base64;

pub use base16::base16_decode;
pub use base32::base32_decode;
pub use base45::base45_decode;
pub use base64::base64_decode;

pub const B64: Base = Base::_64;
pub const B64URL: Base = Base::_64URL;
pub const B32: Base = Base::_32;
pub const B32HEX: Base = Base::_32HEX;
pub const B16: Base = Base::_16;
pub const B45: Base = Base::_45;

// this only exists to match Encoder struct
// otherwise a free function works fine
pub struct Decoder {
    hint: Option<Base>,
}

#[derive(Debug)]
pub enum DecodeError {
    EncodedStringIsCorrupt,
    StringIsNotBaseEncoded,
    StringBaseMismatch,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            hint: Default::default(),
        }
    }

    pub fn hint(mut self, base: Base) -> Self {
        self.hint = Some(base);

        self
    }

    /// this may panic, so may the normal decode method tho
    /// use this when you already know for sure the input string encoding and
    /// want to bypass the encoding guessing step
    pub fn force_decode(value: impl AsRef<str> + Into<String>, base: Base) -> String {
        let value = value.as_ref();
        if value.is_empty() {
            return "".into();
        }

        match base {
            B45 => todo!(),
            B64 | B64URL => base64_decode(value, base),
            B32 => base32_decode(value, base),
            B32HEX => base32_decode(value, base),
            B16 => base16_decode(value, base),
        }
    }

    pub fn decode(&self, value: impl AsRef<str> + Into<String>) -> String {
        let value = value.as_ref();
        if value.is_empty() {
            return "".into();
        }

        let base = self.guess_encoding(value);
        if let Err(e) = base {
            panic!("{:?}", e);
        }
        let base = base.unwrap();
        println!("{:?}", base);

        match base {
            B45 => todo!(),
            B64 | B64URL => base64_decode(value, base),
            B32 | B32HEX => base32_decode(value, base),
            B16 => base16_decode(value, base),
        }
    }

    // deduces the string encoding by process of elimination
    pub(self) fn guess_encoding(&self, value: &str) -> Result<Base, DecodeError> {
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

        let is_32 = chars.clone().all(|c| is_base32(c)) && len % 8 == 0;
        let is_32_hex = chars.clone().all(|c| is_base32_hex(c)) && len % 8 == 0;
        let is_16 = chars.clone().all(|c| c.is_ascii_hexdigit()) && len % 2 == 0;

        if is_32 && is_32_hex {
            if let Some(b) = &self.hint {
                return Ok(*b);
            }
        }

        if is_32 {
            println!("32");
            if chars.clone().any(|c| !is_base32(c)) {
                return Err(DecodeError::EncodedStringIsCorrupt);
            }

            return Ok(Base::_32);
        }
        if is_32_hex {
            println!("32hex");
            if chars.clone().any(|c| !is_base32_hex(c)) {
                return Err(DecodeError::EncodedStringIsCorrupt);
            }

            return Ok(Base::_32HEX);
        }

        if is_16 {
            println!("16");
            if chars.clone().any(|c| !is_base32_hex(c)) {
                return Err(DecodeError::EncodedStringIsCorrupt);
            }

            return Ok(Base::_16);
        }

        Err(DecodeError::StringIsNotBaseEncoded)
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
