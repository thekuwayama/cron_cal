use std::io::{self, prelude::*};
use std::process;
use std::str::FromStr;

mod cli;
mod parse;

const TIME_REQUIRED: usize = 5;

fn main() {
    let matches = cli::build().get_matches();
    let scale = matches
        .value_of(cli::SCALE)
        .expect("Failed to parse scale option");
    let scale: usize = match cli::Scale::from_str(scale).unwrap_or(cli::Scale::Hour) {
        cli::Scale::Quarter => 15,
        cli::Scale::Half => 30,
        cli::Scale::Hour => 60,
    };

    let stdin = io::stdin();
    let mut buf = String::new();
    stdin
        .lock()
        .read_to_string(&mut buf)
        .expect("Failed to read stdin");
    let mut input = buf.as_bytes();
    let cal = parse::parse(&mut input, TIME_REQUIRED).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });
    println!("{}", cli::format_cal(&cal, scale));
}
