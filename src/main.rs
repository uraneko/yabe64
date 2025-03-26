use std::io::BufReader;
use std::io::{Read, stdin};
use yabe64::decoders::base64_decode;

fn main() {
    let stdin = stdin();
    let data = stdin.lines().flatten().collect::<String>();
    if data.is_empty() {
        return;
    }

    println!("{}", base64_decode(data));
}
