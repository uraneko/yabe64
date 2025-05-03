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

    let enc = makura::Encoder::base64_url().encode(&data);

    println!("data = {:?}", data);
    println!("encoded = {:?}", enc);
    println!("base = {:?}", makura::Decoder::deduce_encoding(&enc));
    println!(
        "decoded = {:?}",
        makura::Decoder::decode(&enc, makura::BASE64URL)
    );
    println!(
        "decoded (deduced) = {:?}",
        makura::Decoder::decode_deduce(&enc)
    );
}
