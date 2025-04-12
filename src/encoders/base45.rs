use crate::{Base, PAD, char_from_idx};

const BASE45: Base = Base::_45;

/// separates the input string into chunks of 16bits
fn into_16bits_chunks(data: &str) -> Vec<u16> {
    let mut bytes = data.as_bytes().chunks(2);
    // println!("{:?}", bytes.clone().collect::<Vec<&[u8]>>());
    let last = bytes.next_back().unwrap();

    let mut bytes = bytes
        .map(|b| {
            let mut mask = 0u16;
            mask |= b[0] as u16;
            mask <<= 8;
            mask |= b[1] as u16;

            mask
        })
        .collect::<Vec<u16>>();

    let last = {
        let mut mask = 0u16;
        mask |= last[0] as u16;
        mask <<= 8;
        mask |= if last.len() < 2 { 0u16 } else { last[1] as u16 };

        mask
    };
    bytes.push(last);

    bytes
}

fn into_base45_bytes(bytes: Vec<u16>) -> Vec<u8> {
    let bytes = bytes.into_iter();
    // let mut last = bytes.next_back().unwrap();

    bytes
        .map(|b| {
            println!("{}", b);
            let mut transformer = crate::BaseTransformer::new(45, b);
            transformer.transform_all();

            let seq = transformer.sequence().to_vec();
            println!("{:?}", seq);

            seq
        })
        .flatten()
        .collect()
}

fn into_base45(bytes: Vec<u8>) -> String {
    let bytes = bytes.into_iter();
    let encoded = bytes.map(|b| char_from_idx(b, BASE45)).collect::<String>();

    encoded
}

pub fn base45_encode<T>(value: T) -> String
where
    T: AsRef<str> + Into<String>,
{
    let value = value.as_ref();
    if value.is_empty() {
        return "".into();
    }

    let chunks = into_16bits_chunks(value);
    let bytes = into_base45_bytes(chunks);

    into_base45(bytes)
}
