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

    println!("{}", makura::Encoder::base64().encode(data));
    // println!("{}", makura::Decoder::new().decode(data));
}
