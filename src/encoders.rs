use super::Base;

pub mod base16;
pub mod base32;
pub mod base45;
pub mod base64;

use base16::base16_encode;
use base32::base32_encode;
use base32::base32_hex_encode;
use base45::base45_encode;
use base64::base64_encode;
use base64::base64_url_encode;

pub struct Encoder {
    base: Base,
}

impl Encoder {
    pub fn base64() -> Self {
        Self { base: Base::_64 }
    }
    pub fn base64_url() -> Self {
        Self { base: Base::_64URL }
    }

    pub fn base45() -> Self {
        Self { base: Base::_45 }
    }

    pub fn base32() -> Self {
        Self { base: Base::_32 }
    }
    pub fn base32_hex() -> Self {
        Self { base: Base::_32HEX }
    }
    pub fn base16() -> Self {
        Self { base: Base::_16 }
    }

    pub fn encode(&self, value: impl AsRef<str> + Into<String>) -> String {
        match self.base {
            Base::_45 => base45_encode(value),
            Base::_64 => base64_encode(value),
            Base::_64URL => base64_url_encode(value),
            Base::_32 => base32_encode(value),
            Base::_32HEX => base32_hex_encode(value),
            Base::_16 => base16_encode(value),
        }
    }
}
