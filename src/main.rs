use std::io::{self, prelude::*};
use std::process;
use std::str::FromStr;

use chrono::Utc;

mod cli;
mod format;
mod parse;
mod r#type;

const TIME_REQUIRED: usize = 5;

fn main() {
    // CLI init
    let matches = cli::build().get_matches();
    let scale = matches
        .value_of(cli::SCALE)
        .expect("Failed to parse scale option");
    let scale: usize = match cli::Scale::from_str(scale).unwrap_or(cli::Scale::Hour) {
        cli::Scale::Quarter => 15,
        cli::Scale::Half => 30,
        cli::Scale::Hour => 60,
    };
    // input
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin
        .lock()
        .read_to_string(&mut buf)
        .expect("Failed to read stdin");
    let mut input = buf.as_bytes();
    // parse
    let today = Utc::today().and_hms(0, 0, 0);
    let cal = parse::parse(&mut input, TIME_REQUIRED, today).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });
    // output
    println!("{}", format::format_cal(&cal, scale, today));
}
