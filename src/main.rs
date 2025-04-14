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
    // println!("{:?}\n", makura::Decoder::new().guess_encoding(data));
    // println!("{:?}\n", makura::Decoder::new().decode(data));

    let enc = makura::Encoder::base64().encode(data);
    let (dv, db) = {
        let start = std::time::Instant::now();
        (makura::Decoder::deduce_encoding(&enc), start.elapsed())
    };
    let (gv, gb) = {
        let dec = makura::Decoder::new();
        let start = std::time::Instant::now();
        (dec.guess_encoding(&enc), start.elapsed())
    };
    println!(
        "base64[{}]\ndecode {{ {:?} }} = {:?}\nguess  {{ {:?} }} = {:?}\n",
        enc, db, dv, gb, gv
    );

    let enc = makura::Encoder::base64_url().encode(data);
    let (dv, db) = {
        let start = std::time::Instant::now();
        (makura::Decoder::deduce_encoding(&enc), start.elapsed())
    };

    let (gv, gb) = {
        let dec = makura::Decoder::new();
        let start = std::time::Instant::now();
        (dec.guess_encoding(&enc), start.elapsed())
    };
    println!(
        "base64url[{}]\ndecode {{ {:?} }} = {:?}\nguess  {{ {:?} }} = {:?}\n",
        enc, db, dv, gb, gv
    );

    let enc = makura::Encoder::base45().encode(data);
    let (dv, db) = {
        let start = std::time::Instant::now();
        (makura::Decoder::deduce_encoding(&enc), start.elapsed())
    };

    let (gv, gb) = {
        let dec = makura::Decoder::new();
        let start = std::time::Instant::now();
        (dec.guess_encoding(&enc), start.elapsed())
    };
    println!(
        "base45[{}]\ndecode {{ {:?} }} = {:?}\nguess  {{ {:?} }} = {:?}\n",
        enc, db, dv, gb, gv
    );

    let enc = makura::Encoder::base32().encode(data);
    let (dv, db) = {
        let start = std::time::Instant::now();
        (makura::Decoder::deduce_encoding(&enc), start.elapsed())
    };

    let (gv, gb) = {
        let dec = makura::Decoder::new();
        let start = std::time::Instant::now();
        (dec.guess_encoding(&enc), start.elapsed())
    };
    println!(
        "base32[{}]\ndecode {{ {:?} }} = {:?}\nguess  {{ {:?} }} = {:?}\n",
        enc, db, dv, gb, gv
    );

    let enc = makura::Encoder::base32_hex().encode(data);
    let (dv, db) = {
        let start = std::time::Instant::now();
        (makura::Decoder::deduce_encoding(&enc), start.elapsed())
    };

    let (gv, gb) = {
        let dec = makura::Decoder::new();
        let start = std::time::Instant::now();
        (dec.guess_encoding(&enc), start.elapsed())
    };
    println!(
        "base32hex[{}]\ndecode {{ {:?} }} = {:?}\nguess  {{ {:?} }} = {:?}\n",
        enc, db, dv, gb, gv
    );

    let enc = makura::Encoder::base16().encode(data);
    let (dv, db) = {
        let start = std::time::Instant::now();
        (makura::Decoder::deduce_encoding(&enc), start.elapsed())
    };

    let (gv, gb) = {
        let dec = makura::Decoder::new();
        let start = std::time::Instant::now();
        (dec.guess_encoding(&enc), start.elapsed())
    };
    println!(
        "base16[{}]\ndecode {{ {:?} }} = {:?}\nguess  {{ {:?} }} = {:?}\n",
        enc, db, dv, gb, gv
    );
}
