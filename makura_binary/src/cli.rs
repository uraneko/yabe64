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
    Help {
        message: Help,
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
            "d" | "decode" => Self::Decode {
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
            "e" | "encode" => Self::Encode {
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
            "c" | "convert" => Self::Convert {
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
            "ddc" | "deduce" => Self::Deduce {
                data: if is_term {
                    map.remove("-i")
                        .unwrap_or_else(|| map.remove("--input").unwrap())
                } else {
                    Self::parse_pipe()
                },
            },
            "h" | "help" => CLIInput::Help {
                message: Help::help(),
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
            Self::Help { message } => message.format(),
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

impl Help {
    pub fn help() -> Help {
        Help {
            desc: "base 64, 45, 32, 16 encoding/decoding",
            usage: "mkr [COMMAND] [OPTIONS] [ARGS]",
            commands: vec![
                Cmd {
                    item: "encode",
                    aliases: vec![", e"],
                    desc: "encode given string with the given base encoding",
                    options: vec![],
                },
                Cmd {
                    item: "decode",
                    aliases: vec![", d"],
                    desc: "decode given string from the given base encoding",
                    options: vec![],
                },
                Cmd {
                    item: "convert",
                    aliases: vec![", c"],
                    desc: "takes an encoded string and transforms it to a different encoding",
                    options: vec![
                        Opt {
                            item: "--src",
                            alias: ", -s",
                            desc: "base encoding of the input string to be converted",
                        },
                        Opt {
                            item: "--dest",
                            alias: ", -d",
                            desc: "base encoding of the output string to be generated from source base encoded string",
                        },
                    ],
                },
                Cmd {
                    item: "deduce",
                    aliases: vec![],
                    desc: "tries to deduce the given string's base encoding, may get it wrong",
                    options: vec![],
                },
            ],
            options: vec![
                Opt {
                    item: "--base",
                    alias: ", -b",
                    desc: "base of the encoding/decoding to be applied, used in commands that only need one base option",
                },
                Opt {
                    item: "--input",
                    alias: ", -i",
                    desc: "input string to be processed",
                },
            ],
        }
    }

    fn options(&self) -> Vec<Opt> {
        self.options
            .clone()
            .into_iter()
            .chain(
                self.commands
                    .iter()
                    .map(|c| c.options.clone())
                    .flatten()
                    .collect::<Vec<Opt>>(),
            )
            .collect()
    }

    fn format(&self) -> String {
        format!(
            "{}\n\n{SUP_S}Usage:{RESET_S} {SUB_S}{}{RESET_S}\n\n{SUP_S}Options:{RESET_S} \n{}\n\n{SUP_S}Commands:{RESET_S} \n{}",
            self.desc,
            self.usage,
            self.options()
                .into_iter()
                .map(|o| {
                    let delim = o.item.len() + o.alias.len();
                    let spaces = " ".repeat(18 - delim);

                    format!(
                        "  {SUB_S}{}{}{RESET_S}{}{}\n",
                        o.item, o.alias, spaces, o.desc
                    )
                })
                .collect::<String>(),
            self.commands
                .iter()
                .map(|c| {
                    let aliases = if c.aliases.is_empty() {
                        ""
                    } else {
                        &c.aliases.join(", ")
                    };
                    let delim = c.item.len() + aliases.len();
                    let spaces = " ".repeat(18 - delim);

                    format!(
                        "  {SUB_S}{}{}{RESET_S}{}{}\n",
                        c.item, aliases, spaces, c.desc
                    )
                })
                .collect::<String>()
        )
    }
}

const SUP_S: &str = "\x1b[1;38;2;182;213;132m";
const SUB_S: &str = "\x1b[1;38;2;192;122;113m";
const RESET_S: &str = "\x1b[0m";

#[derive(Debug, Clone)]
pub struct Help {
    desc: &'static str,
    usage: &'static str,
    commands: Vec<Cmd>,
    options: Vec<Opt>,
}

#[derive(Debug, Clone)]
pub struct Cmd {
    item: &'static str,
    aliases: Vec<&'static str>,
    desc: &'static str,
    options: Vec<Opt>,
}

#[derive(Debug, Clone)]
pub struct Opt {
    item: &'static str,
    desc: &'static str,
    alias: &'static str,
}
