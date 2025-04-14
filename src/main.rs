use std::io::BufReader;
use std::io::{Read, stdin};

fn main() {
    let data = stdin().lines().flatten().collect::<String>();
    if data.is_empty() {
        return;
    }

    let data: std::borrow::Cow<str> = std::borrow::Cow::Borrowed(&data);
    let data: String = data.to_string();
    let data: &str = &data;

    println!("{:?}\n", data.as_bytes());

    let enc = makura::Encoder::base64().encode(&data);
    println!("{:?}", makura::Decoder::deduce_encoding(&enc));
    println!("{:?}\n", makura::Decoder::decode(&enc));

    let enc = makura::Encoder::base64_url().encode(&data);
    println!("{:?}", makura::Decoder::deduce_encoding(&enc));
    println!("{:?}\n", makura::Decoder::decode(&enc));

    let enc = makura::Encoder::base45().encode(&data);
    println!("{:?}", makura::Decoder::deduce_encoding(&enc));
    println!("{:?}\n", makura::Decoder::decode(&enc));

    let enc = makura::Encoder::base32().encode(&data);
    println!("{}", enc);
    println!("{:?}", makura::Decoder::deduce_encoding(&enc));
    println!("{:?}\n", makura::Decoder::decode(&enc));

    let enc = makura::Encoder::base32_hex().encode(&data);
    println!("{}", enc);
    println!("{:?}", makura::Decoder::deduce_encoding(&enc));
    println!("{:?}\n", makura::Decoder::decode(&enc));

    let enc = makura::Encoder::base16().encode(&data);
    println!("{:?}", makura::Decoder::deduce_encoding(&enc));
    println!("{:?}\n", makura::Decoder::decode(&enc));
}
