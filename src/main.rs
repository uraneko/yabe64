use makura::base45_decode;
use makura::decoders::Decoder;
use std::io::BufReader;
use std::io::{Read, stdin};

fn main() {
    let stdin = stdin();
    let data = stdin.lines().flatten().collect::<String>();
    if data.is_empty() {
        return;
    }

    println!("{}", base45_decode(&data));
    // println!("{}", Decoder::new().decode(data));
}
