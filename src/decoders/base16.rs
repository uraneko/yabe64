use super::Base;
use super::{into_decoded, into_table_idx};

fn into_8bits_bytes(value: Vec<u8>) -> Vec<u8> {
    value
        .chunks(2)
        .map(|b| {
            let mut mask = 0u8;
            mask |= b[0];
            mask <<= 4;
            mask |= b[1];

            mask
        })
        .collect()
}

pub fn base16_decode(value: &str, base: Base) -> String {
    let indices = into_table_idx(value, &base);
    let bytes = into_8bits_bytes(indices);

    into_decoded(bytes)
}
