pub mod base16;
pub mod base32;
pub mod base64;

pub use base16::base16_encode;
pub use base32::base32_encode;
pub use base32::base32_hex_encode;
pub use base64::base64_encode;
pub use base64::base64_url_encode;

pub const PAD: char = '=';

#[derive(Debug, PartialEq)]
pub enum Base {
    _64,
    _64URL,
    _32,
    _32HEX,
    _16,
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
}

pub fn char_from_idx(idx: u8, base: Base) -> char {
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
        62 if base == Base::_64 => '+',
        63 if base == Base::_64 => '/',

        // NOTE base 64 url is done with this
        62 if base == Base::_64URL => '-',
        63 if base == Base::_64URL => '_',

        // NOTE base 32 is done with tihs
        26 if base.is_32() => '2',
        27 if base.is_32() => '3',
        28 if base.is_32() => '4',
        29 if base.is_32() => '5',
        30 if base.is_32() => '6',
        31 if base.is_32() => '7',

        // hex
        // NOTE base 16 is done with this
        0 if base.hex_16() => '0',
        1 if base.hex_16() => '1',
        2 if base.hex_16() => '2',
        3 if base.hex_16() => '3',
        4 if base.hex_16() => '4',
        5 if base.hex_16() => '5',
        6 if base.hex_16() => '6',
        7 if base.hex_16() => '7',
        8 if base.hex_16() => '8',
        9 if base.hex_16() => '9',
        10 if base.hex_16() => 'A',
        11 if base.hex_16() => 'B',
        12 if base.hex_16() => 'C',
        13 if base.hex_16() => 'D',
        14 if base.hex_16() => 'E',
        15 if base.hex_16() => 'F',

        // NOTE base 32 hex is done with this
        16 if base.is_32_hex() => 'G',
        17 if base.is_32_hex() => 'H',
        18 if base.is_32_hex() => 'I',
        19 if base.is_32_hex() => 'J',
        20 if base.is_32_hex() => 'K',
        21 if base.is_32_hex() => 'L',
        22 if base.is_32_hex() => 'M',
        23 if base.is_32_hex() => 'N',
        24 if base.is_32_hex() => 'O',
        25 if base.is_32_hex() => 'P',
        26 if base.is_32_hex() => 'Q',
        27 if base.is_32_hex() => 'R',
        28 if base.is_32_hex() => 'S',
        29 if base.is_32_hex() => 'T',
        30 if base.is_32_hex() => 'U',
        31 if base.is_32_hex() => 'V',

        _ => panic!("got impossile table index {} for base {:?}", idx, base),
    }
}
