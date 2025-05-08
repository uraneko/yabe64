#![cfg(any(feature = "base64", feature = "base64_url"))]
use crate::makura_alloc::{String, Vec};

use crate::{BASE64, BASE64URL};
use crate::{PAD, char_from_idx};

/// separates the input string into chunks of 24bits
fn into_24bits_chunks(data: &str) -> Vec<u32> {
    let mut bytes = data.as_bytes().chunks(3);
    // println!("{:?}", bytes.clone().collect::<Vec<&[u8]>>());
    let last = bytes.next_back().unwrap();

    let mut bytes = bytes
        .map(|b| {
            let mut mask = 0u32;
            mask |= b[0] as u32;
            mask <<= 8;
            mask |= b[1] as u32;
            mask <<= 8;
            mask |= b[2] as u32;

            mask
        })
        .collect::<Vec<u32>>();
    let last = {
        let mut mask = 0u32;
        mask |= last[0] as u32;
        mask <<= 8;
        mask |= if last.len() < 2 { 0u32 } else { last[1] as u32 };
        mask <<= 8;
        mask |= if last.len() < 3 { 0u32 } else { last[2] as u32 };

        mask
    };
    bytes.push(last);

    bytes
}

fn into_6bits_bytes(bytes: Vec<u32>) -> Vec<u8> {
    let bytes = bytes.into_iter();
    // let mut last = bytes.next_back().unwrap();

    bytes
        .map(|b| {
            [
                (b >> 18) as u8 & 63,
                (b >> 12) as u8 & 63,
                (b >> 6) as u8 & 63,
                b as u8 & 63,
            ]
        })
        .flatten()
        .collect()
}

fn into_base64(bytes: Vec<u8>) -> String {
    let mut bytes = bytes.into_iter();
    let [last, before_last] = [bytes.next_back(), bytes.next_back()];

    // FIXME the table needs to have all values
    let mut encoded = bytes.map(|b| char_from_idx(b, &BASE64)).collect::<String>();

    match [before_last, last] {
        [Some(0), Some(0)] => encoded.extend([PAD, PAD]),
        [Some(b0), Some(0)] => encoded.extend([char_from_idx(b0, &BASE64), PAD]),
        [Some(b0), None] => encoded.push(char_from_idx(b0, &BASE64)),
        [Some(0), Some(b1)] => {
            encoded.extend([char_from_idx(0, &BASE64), char_from_idx(b1, &BASE64)])
        }
        [Some(b0), Some(b1)] => {
            encoded.extend([char_from_idx(b0, &BASE64), char_from_idx(b1, &BASE64)])
        }
        [None, None] => unreachable!("empty vector quit is much earlier"),
        [None, Some(_)] => unreachable!("cant find more data after the end"),
    }

    // if encoded.ends_with("AA") {
    // } else if encoded.ends_with('A') {
    // }

    encoded
}

fn into_base64_url(bytes: Vec<u8>) -> String {
    let mut bytes = bytes.into_iter();
    let [last, before_last] = [bytes.next_back(), bytes.next_back()];

    // FIXME the table needs to have all values
    let mut encoded = bytes
        .map(|b| char_from_idx(b, &BASE64URL))
        .collect::<String>();

    match [before_last, last] {
        [Some(0), Some(0)] => encoded.extend([PAD, PAD]),
        [Some(b0), Some(0)] => encoded.extend([char_from_idx(b0, &BASE64URL), PAD]),
        [Some(b0), None] => encoded.push(char_from_idx(b0, &BASE64URL)),
        [Some(0), Some(b1)] => {
            encoded.extend([char_from_idx(0, &BASE64URL), char_from_idx(b1, &BASE64URL)])
        }
        [Some(b0), Some(b1)] => {
            encoded.extend([char_from_idx(b0, &BASE64URL), char_from_idx(b1, &BASE64URL)])
        }
        [None, None] => unreachable!("empty vector quit is much earlier"),
        [None, Some(_)] => unreachable!("cant find more data after the end"),
    }

    // if encoded.ends_with("AA") {
    // } else if encoded.ends_with('A') {
    // }

    encoded
}

#[cfg(feature = "base64")]
pub fn base64_encode<T>(value: T) -> String
where
    T: AsRef<str>,
{
    let value = value.as_ref();
    if value.is_empty() {
        return "".into();
    }

    let chunks = into_24bits_chunks(value);
    let bytes = into_6bits_bytes(chunks);

    into_base64(bytes)
}

#[cfg(feature = "base64_url")]
pub fn base64_url_encode<T>(value: T) -> String
where
    T: AsRef<str>,
{
    let value = value.as_ref();
    if value.is_empty() {
        return "".into();
    }

    let chunks = into_24bits_chunks(value);
    let bytes = into_6bits_bytes(chunks);

    into_base64_url(bytes)
}
