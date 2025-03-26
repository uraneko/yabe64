use super::Base;

pub mod base16;
pub mod base32;
pub mod base64;

pub use base64::base64_decode;

pub const B64: Base = Base::_64;
pub const B64URL: Base = Base::_64URL;
pub const B32: Base = Base::_32;
pub const B32HEX: Base = Base::_32HEX;
pub const B16: Base = Base::_16;

pub struct Decoder {
    hint: Base,
}

#[derive(Debug)]
pub enum DecodeError {
    EncodedStringIsCorrupt,
    StringIsNotBaseEncoded,
    StringBaseMismatch,
}

impl Decoder {
    pub fn hint(mut self, base: Base) -> Self {
        self.hint = base;

        self
    }

    pub fn force_decode(&self, base: Base) -> Result<String, DecodeError> {
        // TODO call decoding functions
        match base {
            B64 => {}
            B64URL => {}
            B32 => {}
            B32HEX => {}
            B16 => {}
        }

        unimplemented!("later");
    }
}
