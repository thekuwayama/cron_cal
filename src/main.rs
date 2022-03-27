use std::io::{self, prelude::*};
use std::process;
use std::str::FromStr;

use chrono::naive::NaiveDate;
use chrono::{DateTime, Utc};

mod cli;
mod format;
mod parse;
mod r#type;

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
    let spare = matches.is_present(cli::SPARE);
    let date = matches
        .value_of(cli::DATE)
        .map(|s| {
            NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map(|n| DateTime::<Utc>::from_utc(n.and_hms(0, 0, 0), Utc))
        })
        .unwrap_or_else(|| Ok(Utc::today().and_hms(0, 0, 0)))
        .unwrap_or_else(|e| {
            eprintln!("Failed to parse date option: {}", e);
            process::exit(1);
        });
    // Read input
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_to_string(&mut buf).unwrap_or_else(|e| {
        eprintln!("Failed to read stdin: {}", e);
        process::exit(1);
    });
    let mut input = buf.as_bytes();
    // Parse input
    let cal = parse::parse(&mut input, date).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });
    // Print result
    println!(
        "{}",
        if spare {
            format::format_cal_spare(&cal, scale, date)
        } else {
            format::format_cal(&cal, scale, date)
        }
    );
}
