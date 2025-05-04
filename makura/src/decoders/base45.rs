#![cfg(feature = "base45")]
use crate::makura_alloc::Vec;

use super::DecodeError;

fn into_base45_values(bytes: Vec<u8>) -> Vec<u16> {
    let mut chunks = bytes.chunks(3);
    let last = chunks.next_back().unwrap();

    let mut values: Vec<u16> = chunks
        // .inspect(|c| println!("{:?}", c))
        .map(|b| b[2] as u16 * 45 * 45 + b[1] as u16 * 45 + b[0] as u16)
        .collect();

    let last = {
        if last.len() == 3 {
            last[2] as u16 * 45 * 45 + last[1] as u16 * 45 + last[0] as u16
        } else if last.len() == 2 {
            last[1] as u16 * 45 + last[0] as u16
        } else {
            unreachable!("last chunk len can only be 2 or 3");
        }
    };
    values.push(last);

    values
}

// get back 8 bit bytes from the 24bits bytes
fn into_base265_values(value: Vec<u16>) -> Vec<u8> {
    let mut bytes = value.into_iter();
    let last = bytes.next_back().unwrap();
    let mut bytes = bytes
        .map(|b| [((b & 0xff00) >> 8) as u8, b as u8])
        .flatten()
        .collect::<Vec<u8>>();

    if last < u8::MAX as u16 {
        bytes.push(last as u8);
    } else {
        bytes.push((last >> 8) as u8);
        bytes.push(last as u8);
    }

    bytes
}

pub fn base45_decode(indices: Vec<u8>) -> Vec<u8> {
    let bytes = into_base45_values(indices);

    into_base265_values(bytes)
}
