use std::io::{BufRead, BufReader, IsTerminal};
use std::io::{Read, stdin};

use makura::{Decoder, Encoder};

// TODO this is quirky
// make it more erginimic
#[path = "mkr/cli.rs"]
mod cli;

fn main() {
    // let args = std::env::args();
    // println!("{:?}", args);

    let input = cli::CLIInput::new();
    // println!("{:#?}", input);

    input.run();
}
