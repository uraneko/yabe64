pub mod base64;
pub(crate) mod root;

pub use base64::base64_encode;

use std::io::BufReader;
use std::io::{Read, stdin};

fn main() {
    let stdin = stdin();
    let data = stdin.lines().flatten().collect::<String>();
    if data.is_empty() {
        return;
    }

    println!("{}", base64_encode(data));
}
