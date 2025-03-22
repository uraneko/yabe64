use crate::root::{BASE64_PAD as PAD, BASE64_TABLE as TABLE};

/// DOCS
/// last 3 octets
/// (1) The final quantum of encoding input is an integral multiple of 24
///     bits; here, the final unit of encoded output will be an integral
///     multiple of 4 characters with no "=" padding.
///
/// (2) The final quantum of encoding input is exactly 8 bits; here, the
///     final unit of encoded output will be two characters followed by
///     two "=" padding characters.
///
/// (3) The final quantum of encoding input is exactly 16 bits; here, the
///     final unit of encoded output will be three characters followed by
///     one "=" padding character.

/// separates the input string into chunks of 24bits
pub fn into_24bits_chunks<T>(data: T) -> Vec<u32>
where
    T: AsRef<str> + Into<String>,
{
    let data = data.as_ref();
    if data.is_empty() {
        return vec![];
    }
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

pub fn into_6bits_bytes(bytes: Vec<u32>) -> Vec<u8> {
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

pub fn into_base64(bytes: Vec<u8>) -> String {
    let mut bytes = bytes.into_iter();
    let [last, before_last] = [bytes.next_back(), bytes.next_back()];

    let mut encoded = bytes.map(|b| *TABLE.get(&b).unwrap()).collect::<String>();

    match [before_last, last] {
        [Some(0), Some(0)] => encoded.extend([PAD, PAD]),
        [Some(b0), Some(0)] => encoded.extend([TABLE.get(&b0).unwrap(), &PAD]),
        [Some(b0), None] => encoded.push(*TABLE.get(&b0).unwrap()),
        [Some(0), Some(b1)] => encoded.extend([TABLE.get(&0).unwrap(), TABLE.get(&b1).unwrap()]),
        [Some(b0), Some(b1)] => encoded.extend([TABLE.get(&b0).unwrap(), TABLE.get(&b1).unwrap()]),
        [None, None] => unreachable!("empty vector quit is much earlier"),
        [None, Some(_)] => unreachable!("cant find more data after the end"),
    }

    // if encoded.ends_with("AA") {
    // } else if encoded.ends_with('A') {
    // }

    encoded
}

pub fn base64_encode<T>(value: T) -> String
where
    T: AsRef<str> + Into<String>,
{
    let chunks = into_24bits_chunks(value);
    let bytes = into_6bits_bytes(chunks);

    into_base64(bytes)
}
