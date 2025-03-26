use std::io::BufReader;
use std::io::{Read, stdin};
use yabe64::decoders::Decoder;

fn main() {
    let stdin = stdin();
    let data = stdin.lines().flatten().collect::<String>();
    if data.is_empty() {
        return;
    }

    println!("{}", Decoder::new().decode(data));
}
