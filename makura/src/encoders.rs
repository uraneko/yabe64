#![cfg(feature = "encoding")]
use crate::makura_alloc::String;

use super::Base;
use super::{BASE16, BASE32, BASE32HEX, BASE45, BASE64, BASE64URL};

mod base16;
mod base32;
mod base45;
mod base64;

use base16::base16_encode;
use base32::base32_encode;
use base32::base32_hex_encode;
use base45::base45_encode;
use base64::base64_encode;
use base64::base64_url_encode;

/// exposes feature enabled bases encoding
pub struct Encoder {
    base: Base,
}

impl Encoder {
    /// returns the base of the encoder
    pub fn base(&self) -> &Base {
        &self.base
    }

    #[cfg(feature = "base64")]
    /// creates a new base64 encoder
    pub fn base64() -> Self {
        Self { base: Base::_64 }
    }

    #[cfg(feature = "base64_url")]
    /// creates a new base64 url encoder
    pub fn base64_url() -> Self {
        Self { base: Base::_64URL }
    }

    #[cfg(feature = "base45")]
    /// creates a new base45 encoder
    pub fn base45() -> Self {
        Self { base: Base::_45 }
    }

    #[cfg(feature = "base32")]
    /// creates a new base32 encoder
    pub fn base32() -> Self {
        Self { base: Base::_32 }
    }

    #[cfg(feature = "base32_hex")]
    /// creates a new base32 hex encoder
    pub fn base32_hex() -> Self {
        Self { base: Base::_32HEX }
    }

    #[cfg(feature = "base16")]
    /// creates a new base16 encoder
    pub fn base16() -> Self {
        Self { base: Base::_16 }
    }

    /// Apply self's base encoding to passed value argument.
    /// Value can be anything that implements `AsRef<str>`;
    /// including an `&str`, an owned `String` or a `Cow<str>`
    ///
    /// This method always returns a string,
    /// passing an empty string results in a an empty `String` return value
    pub fn encode(&self, value: impl AsRef<str>) -> String {
        match self.base {
            Base::_64 => base64_encode(value),
            Base::_64URL => base64_url_encode(value),
            Base::_45 => base45_encode(value),
            Base::_32 => base32_encode(value),
            Base::_32HEX => base32_hex_encode(value),
            Base::_16 => base16_encode(value),
        }
    }
}

impl From<Base> for Encoder {
    fn from(value: Base) -> Self {
        match value {
            BASE64 => Encoder::base64(),
            BASE64URL => Encoder::base64_url(),
            BASE45 => Encoder::base45(),
            BASE32 => Encoder::base32(),
            BASE32HEX => Encoder::base32_hex(),
            BASE16 => Encoder::base16(),
        }
    }
}
