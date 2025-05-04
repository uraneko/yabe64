use std::collections::HashMap;
use std::io::{BufRead, BufReader, IsTerminal, Write};
use std::io::{Read, stdin};

use makura::Base;
use makura::{Decoder, Encoder};

#[derive(Debug)]
pub enum CLIInput {
    Decode {
        base: Base,
        data: String,
        auto: bool,
    },
    Encode {
        base: Base,
        data: String,
    },
    Convert {
        src: Base,
        dest: Base,
        data: String,
    },
    Deduce {
        data: String,
    },
}

impl CLIInput {
    pub fn new() -> Self {
        Self::parse_args(stdin().is_terminal())
    }

    fn parse_args(is_term: bool) -> Self {
        let mut args = std::env::args();
        args.next();

        let action = args.next().unwrap();
        let mut map: HashMap<String, String> = HashMap::with_capacity(4);
        while let Some(arg) = args.next() {
            map.insert(arg, args.next().unwrap());
        }

        match action.trim() {
            "-D" | "--decode" => Self::Decode {
                data: if is_term {
                    map.remove("-i")
                        .unwrap_or_else(|| map.remove("--input").unwrap())
                } else {
                    Self::parse_pipe()
                },
                base: (map
                    .remove("-b")
                    .unwrap_or_else(|| map.remove("--base").unwrap()))
                .as_str()
                .try_into()
                .unwrap(),
                auto: if map.contains_key("--auto") || map.contains_key("-a") {
                    true
                } else {
                    false
                },
            },
            "-E" | "--encode" => Self::Encode {
                data: if is_term {
                    map.remove("-i")
                        .unwrap_or_else(|| map.remove("--input").unwrap())
                } else {
                    Self::parse_pipe()
                },
                base: (map
                    .remove("-b")
                    .unwrap_or_else(|| map.remove("--base").unwrap()))
                .as_str()
                .try_into()
                .unwrap(),
            },
            "-C" | "--convert" => Self::Convert {
                data: if is_term {
                    map.remove("-i")
                        .unwrap_or_else(|| map.remove("--input").unwrap())
                } else {
                    Self::parse_pipe()
                },
                src: (map
                    .remove("-s")
                    .unwrap_or_else(|| map.remove("--src").unwrap()))
                .as_str()
                .try_into()
                .unwrap(),
                dest: (map
                    .remove("-d")
                    .unwrap_or_else(|| map.remove("--dest").unwrap()))
                .as_str()
                .try_into()
                .unwrap(),
            },
            // -G for guess
            "-G" | "--guess" => Self::Deduce {
                data: if is_term {
                    map.remove("-i")
                        .unwrap_or_else(|| map.remove("--input").unwrap())
                } else {
                    Self::parse_pipe()
                },
            },
            val => unreachable!("{}", val),
        }
    }

    fn parse_pipe() -> String {
        stdin().lock().lines().flatten().collect::<String>()
    }

    // TODO add cli help message

    pub fn run(self) {
        let mut stdout = std::io::stdout().lock();
        let output = match self {
            Self::Decode { data, base, auto } => {
                let output = if auto {
                    Decoder::decode_deduce(data)
                } else {
                    Decoder::decode(data, base)
                };

                if let Ok(o) = output {
                    o
                } else {
                    panic!("{:?}", output)
                }
            }
            Self::Encode { data, base } => {
                let enc = Encoder::from(base);

                enc.encode(data)
            }
            Self::Convert { data, src, dest } => {
                let input = Decoder::decode(data, src);
                if input.is_err() {
                    panic!("{:#?}", input);
                }
                let input = input.unwrap();
                let enc = Encoder::from(dest);

                enc.encode(input)
            }
            Self::Deduce { data } => {
                let base = Decoder::deduce_encoding(&data);

                format!("{:#?}", base)
            }
        };

        stdout.write(output.as_bytes()).unwrap();
    }
}

// enum CLIError {
//     NotEnoughArgs,
// }
