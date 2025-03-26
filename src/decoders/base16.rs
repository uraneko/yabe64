use crate::{Base, PAD, char_from_idx};

const BASE16: Base = Base::_16;

/// separates the input string into chunks of 24bits
fn into_octets(data: &str) -> Vec<u8> {
    data.into()
}

fn into_4bits_bytes(bytes: Vec<u8>) -> Vec<u8> {
    let bytes = bytes.into_iter();
    // let mut last = bytes.next_back().unwrap();

    bytes.map(|b| [(b >> 4) & 15, b & 15]).flatten().collect()
}

fn into_base16(bytes: Vec<u8>) -> String {
    bytes
        .into_iter()
        .map(|b| char_from_idx(b, BASE16))
        .collect::<String>()
}

pub fn base16_encode<T>(value: T) -> String
where
    T: AsRef<str> + Into<String>,
{
    let value = value.as_ref();
    if value.is_empty() {
        return "".into();
    }

    let chunks = into_octets(value);
    let bytes = into_4bits_bytes(chunks);

    into_base16(bytes)
}
