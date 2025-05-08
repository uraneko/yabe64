#![no_std]
#![doc(html_playground_url = "https://play.rust-lang.org/?version=stable&mode=debug&edition=2024")]
#![cfg_attr(feature = "nightly", feature(doc_auto_cfg))]
#![cfg_attr(feature = "nightly", feature(test))]

mod base_transformer;
pub(crate) use base_transformer::BaseTransformer;

mod decoders;
mod encoders;

pub use decoders::Bases;
pub use decoders::DecodeError;
pub use decoders::Decoder;
pub use encoders::Encoder;

pub(crate) const PAD: char = '=';

pub const BASE64: Base = Base::_64;
pub const BASE64URL: Base = Base::_64URL;
pub const BASE32: Base = Base::_32;
pub const BASE32HEX: Base = Base::_32HEX;
pub const BASE16: Base = Base::_16;
pub const BASE45: Base = Base::_45;

#[derive(PartialEq, Clone, Copy, Ord, PartialOrd, Eq)]
pub enum Base {
    _64,
    _64URL,
    _45,
    _32,
    _32HEX,
    _16,
}

impl core::fmt::Debug for Base {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::_64 => "Base64",
                Self::_64URL => "Base64URL",
                Self::_45 => "Base45",
                Self::_32 => "Base32",
                Self::_32HEX => "Base32HEX",
                Self::_16 => "Base16",
            }
        )
    }
}

impl core::fmt::Display for Base {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::_64 => "Base64",
                Self::_64URL => "Base64URL",
                Self::_45 => "Base45",
                Self::_32 => "Base32",
                Self::_32HEX => "Base32HEX",
                Self::_16 => "Base16",
            }
        )
    }
}

impl TryFrom<&str> for Base {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "64" => Ok(BASE64),
            "64url" => Ok(BASE64URL),
            "45" => Ok(BASE45),
            "32" => Ok(BASE32),
            "32hex" => Ok(BASE32HEX),
            "16" => Ok(BASE16),
            _ => Err(()),
        }
    }
}

impl Base {
    // first 26 values of base encoding table are the uppercase alphabet letters A -> Z
    fn alpha_26(&self) -> bool {
        self == &Self::_64 || self == &Self::_64URL || self == &Self::_32
    }

    // first 16 values in the base encoding table are the base 16 numbers 0 -> F
    fn hex_16(&self) -> bool {
        self == &Self::_32HEX || self == &Self::_16
    }

    // base is 64 or 64 url
    fn is_any_64(&self) -> bool {
        self == &Self::_64 || self == &Self::_64URL
    }

    // base is strictly 32
    fn is_32(&self) -> bool {
        self == &Self::_32
    }

    // base is strictly 32 hex
    fn is_32_hex(&self) -> bool {
        self == &Self::_32HEX
    }

    fn is_45(&self) -> bool {
        self == &Self::_45
    }
}

pub(crate) fn char_from_idx(idx: u8, base: &Base) -> char {
    // NOTE handled by the panic below
    // if idx > 63 {
    //     return Err(DecodeError::TableIndexOverflow(idx));
    // }

    match idx {
        // alpha
        0 if base.alpha_26() => 'A',
        1 if base.alpha_26() => 'B',
        2 if base.alpha_26() => 'C',
        3 if base.alpha_26() => 'D',
        4 if base.alpha_26() => 'E',
        5 if base.alpha_26() => 'F',
        6 if base.alpha_26() => 'G',
        7 if base.alpha_26() => 'H',
        8 if base.alpha_26() => 'I',
        9 if base.alpha_26() => 'J',
        10 if base.alpha_26() => 'K',
        11 if base.alpha_26() => 'L',
        12 if base.alpha_26() => 'M',
        13 if base.alpha_26() => 'N',
        14 if base.alpha_26() => 'O',
        15 if base.alpha_26() => 'P',
        16 if base.alpha_26() => 'Q',
        17 if base.alpha_26() => 'R',
        18 if base.alpha_26() => 'S',
        19 if base.alpha_26() => 'T',
        20 if base.alpha_26() => 'U',
        21 if base.alpha_26() => 'V',
        22 if base.alpha_26() => 'W',
        23 if base.alpha_26() => 'X',
        24 if base.alpha_26() => 'Y',
        25 if base.alpha_26() => 'Z',

        26 if base.is_any_64() => 'a',
        27 if base.is_any_64() => 'b',
        28 if base.is_any_64() => 'c',
        29 if base.is_any_64() => 'd',
        30 if base.is_any_64() => 'e',
        31 if base.is_any_64() => 'f',
        32 if base.is_any_64() => 'g',
        33 if base.is_any_64() => 'h',
        34 if base.is_any_64() => 'i',
        35 if base.is_any_64() => 'j',
        36 if base.is_any_64() => 'k',
        37 if base.is_any_64() => 'l',
        38 if base.is_any_64() => 'm',
        39 if base.is_any_64() => 'n',
        40 if base.is_any_64() => 'o',
        41 if base.is_any_64() => 'p',
        42 if base.is_any_64() => 'q',
        43 if base.is_any_64() => 'r',
        44 if base.is_any_64() => 's',
        45 if base.is_any_64() => 't',
        46 if base.is_any_64() => 'u',
        47 if base.is_any_64() => 'v',
        48 if base.is_any_64() => 'w',
        49 if base.is_any_64() => 'x',
        50 if base.is_any_64() => 'y',
        51 if base.is_any_64() => 'z',
        52 if base.is_any_64() => '0',
        53 if base.is_any_64() => '1',
        54 if base.is_any_64() => '2',
        55 if base.is_any_64() => '3',
        56 if base.is_any_64() => '4',
        57 if base.is_any_64() => '5',
        58 if base.is_any_64() => '6',
        59 if base.is_any_64() => '7',
        60 if base.is_any_64() => '8',
        61 if base.is_any_64() => '9',

        // NOTE base 64 is done with this
        62 if base == &Base::_64 => '+',
        63 if base == &Base::_64 => '/',

        // NOTE base 64 url is done with this
        62 if base == &Base::_64URL => '-',
        63 if base == &Base::_64URL => '_',

        // NOTE base 32 is done with tihs
        26 if base.is_32() => '2',
        27 if base.is_32() => '3',
        28 if base.is_32() => '4',
        29 if base.is_32() => '5',
        30 if base.is_32() => '6',
        31 if base.is_32() => '7',

        // hex
        // NOTE base 16 is done with this
        0 if base.hex_16() | base.is_45() => '0',
        1 if base.hex_16() | base.is_45() => '1',
        2 if base.hex_16() | base.is_45() => '2',
        3 if base.hex_16() | base.is_45() => '3',
        4 if base.hex_16() | base.is_45() => '4',
        5 if base.hex_16() | base.is_45() => '5',
        6 if base.hex_16() | base.is_45() => '6',
        7 if base.hex_16() | base.is_45() => '7',
        8 if base.hex_16() | base.is_45() => '8',
        9 if base.hex_16() | base.is_45() => '9',
        10 if base.hex_16() | base.is_45() => 'A',
        11 if base.hex_16() | base.is_45() => 'B',
        12 if base.hex_16() | base.is_45() => 'C',
        13 if base.hex_16() | base.is_45() => 'D',
        14 if base.hex_16() | base.is_45() => 'E',
        15 if base.hex_16() | base.is_45() => 'F',

        // NOTE base 32 hex is done with this
        16 if base.is_32_hex() | base.is_45() => 'G',
        17 if base.is_32_hex() | base.is_45() => 'H',
        18 if base.is_32_hex() | base.is_45() => 'I',
        19 if base.is_32_hex() | base.is_45() => 'J',
        20 if base.is_32_hex() | base.is_45() => 'K',
        21 if base.is_32_hex() | base.is_45() => 'L',
        22 if base.is_32_hex() | base.is_45() => 'M',
        23 if base.is_32_hex() | base.is_45() => 'N',
        24 if base.is_32_hex() | base.is_45() => 'O',
        25 if base.is_32_hex() | base.is_45() => 'P',
        26 if base.is_32_hex() | base.is_45() => 'Q',
        27 if base.is_32_hex() | base.is_45() => 'R',
        28 if base.is_32_hex() | base.is_45() => 'S',
        29 if base.is_32_hex() | base.is_45() => 'T',
        30 if base.is_32_hex() | base.is_45() => 'U',
        31 if base.is_32_hex() | base.is_45() => 'V',

        32 if base.is_45() => 'W',
        33 if base.is_45() => 'X',
        34 if base.is_45() => 'Y',
        35 if base.is_45() => 'Z',
        36 if base.is_45() => ' ',
        37 if base.is_45() => '$',
        38 if base.is_45() => '%',
        39 if base.is_45() => '*',
        40 if base.is_45() => '+',
        41 if base.is_45() => '-',
        42 if base.is_45() => '.',
        43 if base.is_45() => '/',
        44 if base.is_45() => ':',

        // NOTE let this panic, since this crate is always in control of the u8 that
        // this function receives, guarenteeing that this match arm is always unreachable
        idx => unreachable!("got impossile table index {} for base {:?}", idx, base),
    }
}

pub(crate) fn idx_from_char(chr: char, base: &Base) -> Result<u8, DecodeError> {
    match chr {
        // alpha
        'A' if base.alpha_26() => Ok(0),
        'B' if base.alpha_26() => Ok(1),
        'C' if base.alpha_26() => Ok(2),
        'D' if base.alpha_26() => Ok(3),
        'E' if base.alpha_26() => Ok(4),
        'F' if base.alpha_26() => Ok(5),
        'G' if base.alpha_26() => Ok(6),
        'H' if base.alpha_26() => Ok(7),
        'I' if base.alpha_26() => Ok(8),
        'J' if base.alpha_26() => Ok(9),
        'K' if base.alpha_26() => Ok(10),
        'L' if base.alpha_26() => Ok(11),
        'M' if base.alpha_26() => Ok(12),
        'N' if base.alpha_26() => Ok(13),
        'O' if base.alpha_26() => Ok(14),
        'P' if base.alpha_26() => Ok(15),
        'Q' if base.alpha_26() => Ok(16),
        'R' if base.alpha_26() => Ok(17),
        'S' if base.alpha_26() => Ok(18),
        'T' if base.alpha_26() => Ok(19),
        'U' if base.alpha_26() => Ok(20),
        'V' if base.alpha_26() => Ok(21),
        'W' if base.alpha_26() => Ok(22),
        'X' if base.alpha_26() => Ok(23),
        'Y' if base.alpha_26() => Ok(24),
        'Z' if base.alpha_26() => Ok(25),

        'a' if base.is_any_64() => Ok(26),
        'b' if base.is_any_64() => Ok(27),
        'c' if base.is_any_64() => Ok(28),
        'd' if base.is_any_64() => Ok(29),
        'e' if base.is_any_64() => Ok(30),
        'f' if base.is_any_64() => Ok(31),
        'g' if base.is_any_64() => Ok(32),
        'h' if base.is_any_64() => Ok(33),
        'i' if base.is_any_64() => Ok(34),
        'j' if base.is_any_64() => Ok(35),
        'k' if base.is_any_64() => Ok(36),
        'l' if base.is_any_64() => Ok(37),
        'm' if base.is_any_64() => Ok(38),
        'n' if base.is_any_64() => Ok(39),
        'o' if base.is_any_64() => Ok(40),
        'p' if base.is_any_64() => Ok(41),
        'q' if base.is_any_64() => Ok(42),
        'r' if base.is_any_64() => Ok(43),
        's' if base.is_any_64() => Ok(44),
        't' if base.is_any_64() => Ok(45),
        'u' if base.is_any_64() => Ok(46),
        'v' if base.is_any_64() => Ok(47),
        'w' if base.is_any_64() => Ok(48),
        'x' if base.is_any_64() => Ok(49),
        'y' if base.is_any_64() => Ok(50),
        'z' if base.is_any_64() => Ok(51),
        '0' if base.is_any_64() => Ok(52),
        '1' if base.is_any_64() => Ok(53),
        '2' if base.is_any_64() => Ok(54),
        '3' if base.is_any_64() => Ok(55),
        '4' if base.is_any_64() => Ok(56),
        '5' if base.is_any_64() => Ok(57),
        '6' if base.is_any_64() => Ok(58),
        '7' if base.is_any_64() => Ok(59),
        '8' if base.is_any_64() => Ok(60),
        '9' if base.is_any_64() => Ok(61),

        // NOTE base 64 is done with this
        '+' if base == &Base::_64 => Ok(62),
        '/' if base == &Base::_64 => Ok(63),

        // NOTE base 64 url is done with this
        '-' if base == &Base::_64URL => Ok(62),
        '_' if base == &Base::_64URL => Ok(63),

        // NOTE base 32 is done with this
        '2' if base.is_32() => Ok(26),
        '3' if base.is_32() => Ok(27),
        '4' if base.is_32() => Ok(28),
        '5' if base.is_32() => Ok(29),
        '6' if base.is_32() => Ok(30),
        '7' if base.is_32() => Ok(31),

        // hex
        // NOTE base 16 is done with this
        '0' if base.hex_16() | base.is_45() => Ok(0),
        '1' if base.hex_16() | base.is_45() => Ok(1),
        '2' if base.hex_16() | base.is_45() => Ok(2),
        '3' if base.hex_16() | base.is_45() => Ok(3),
        '4' if base.hex_16() | base.is_45() => Ok(4),
        '5' if base.hex_16() | base.is_45() => Ok(5),
        '6' if base.hex_16() | base.is_45() => Ok(6),
        '7' if base.hex_16() | base.is_45() => Ok(7),
        '8' if base.hex_16() | base.is_45() => Ok(8),
        '9' if base.hex_16() | base.is_45() => Ok(9),
        'A' if base.hex_16() | base.is_45() => Ok(10),
        'B' if base.hex_16() | base.is_45() => Ok(11),
        'C' if base.hex_16() | base.is_45() => Ok(12),
        'D' if base.hex_16() | base.is_45() => Ok(13),
        'E' if base.hex_16() | base.is_45() => Ok(14),
        'F' if base.hex_16() | base.is_45() => Ok(15),

        // NOTE base 32 hex is done with this
        'G' if base.is_32_hex() | base.is_45() => Ok(16),
        'H' if base.is_32_hex() | base.is_45() => Ok(17),
        'I' if base.is_32_hex() | base.is_45() => Ok(18),
        'J' if base.is_32_hex() | base.is_45() => Ok(19),
        'K' if base.is_32_hex() | base.is_45() => Ok(20),
        'L' if base.is_32_hex() | base.is_45() => Ok(21),
        'M' if base.is_32_hex() | base.is_45() => Ok(22),
        'N' if base.is_32_hex() | base.is_45() => Ok(23),
        'O' if base.is_32_hex() | base.is_45() => Ok(24),
        'P' if base.is_32_hex() | base.is_45() => Ok(25),
        'Q' if base.is_32_hex() | base.is_45() => Ok(26),
        'R' if base.is_32_hex() | base.is_45() => Ok(27),
        'S' if base.is_32_hex() | base.is_45() => Ok(28),
        'T' if base.is_32_hex() | base.is_45() => Ok(29),
        'U' if base.is_32_hex() | base.is_45() => Ok(30),
        'V' if base.is_32_hex() | base.is_45() => Ok(31),

        'W' if base.is_45() => Ok(32),
        'X' if base.is_45() => Ok(33),
        'Y' if base.is_45() => Ok(34),
        'Z' if base.is_45() => Ok(35),
        ' ' if base.is_45() => Ok(36),
        '$' if base.is_45() => Ok(37),
        '%' if base.is_45() => Ok(38),
        '*' if base.is_45() => Ok(39),
        '+' if base.is_45() => Ok(40),
        '-' if base.is_45() => Ok(41),
        '.' if base.is_45() => Ok(42),
        '/' if base.is_45() => Ok(43),
        ':' if base.is_45() => Ok(44),

        // _ => panic!("got impossile table char {} for base {:?}", chr, base),
        ch => Err(DecodeError::UnrecognizedCharForBase { ch, base: *base }),
    }
}

pub(crate) mod makura_alloc {
    extern crate alloc;
    pub(crate) use alloc::borrow::Cow;
    pub(crate) use alloc::collections::BTreeSet;
    pub(crate) use alloc::string::String;
    pub(crate) use alloc::{vec, vec::Vec};
}

pub(crate) mod makura_core {
    pub(crate) use core::ops;
    pub(crate) use core::str::Utf8Error;
}

#[cfg(test)]
mod tests {
    use core::assert;

    use super::char_from_idx;
    use super::{BASE16, BASE32, BASE45, BASE64};

    #[test]
    #[should_panic]
    fn fail_char_from_idx64() {
        char_from_idx(64, &BASE64);

        assert!(true);
    }

    #[test]
    #[should_panic]
    fn fail_char_from_idx45() {
        char_from_idx(45, &BASE45);

        assert!(true);
    }

    #[test]
    #[should_panic]
    fn fail_char_from_idx32() {
        char_from_idx(32, &BASE32);

        assert!(true);
    }

    #[test]
    #[should_panic]
    fn fail_char_from_idx16() {
        char_from_idx(16, &BASE16);

        assert!(true);
    }

    #[test]
    fn test_char_from_idx64() {
        char_from_idx(63, &BASE64);

        assert!(true);
    }

    #[test]
    fn test_char_from_idx45() {
        char_from_idx(44, &BASE45);

        assert!(true);
    }

    #[test]
    fn test_char_from_idx32() {
        char_from_idx(31, &BASE32);

        assert!(true);
    }

    #[test]
    fn test_char_from_idx16() {
        char_from_idx(15, &BASE16);

        assert!(true);
    }
}
