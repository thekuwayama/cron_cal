use std::io::{self, prelude::*};
use std::process;

use chrono::{NaiveDate, NaiveDateTime, TimeZone, Utc};

mod cli;
mod format;
mod parse;
mod r#type;

fn main() {
    // CLI init
    let now = Utc::now();
    let matches = cli::build(&now.format("%F").to_string()).get_matches();
    let date = matches
        .value_of(cli::DATE)
        .map(|s| {
            NaiveDate::parse_from_str(s, "%F")
                .map(|n| Utc.from_utc_datetime(&n.and_hms_opt(0, 0, 0).unwrap()))
                .unwrap_or_else(|e| {
                    eprintln!("Failed to parse date option: {}", e);
                    process::exit(1);
                })
        })
        .unwrap_or_else(|| now);
    let days = matches
        .value_of(cli::DAYS)
        .map(|s| {
            s.parse::<usize>().unwrap_or_else(|e| {
                eprintln!("Failed to parse days option: {}", e);
                process::exit(1);
            })
        })
        .unwrap_or(1);
    // Read input
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_to_string(&mut buf).unwrap_or_else(|e| {
        eprintln!("Failed to read stdin: {}", e);
        process::exit(1);
    });
    let mut input = buf.as_bytes();
    // Parse input
    let cal = parse::parse(&mut input, date, days).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });
    // Print result
    format::format_unix_timestamp(&cal, date)
        .iter()
        .for_each(|p| {
            println!(
                "{} ~ {}",
                Utc.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(p.start, 0).unwrap())
                    .format("%F %R"),
                Utc.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(p.end, 0).unwrap())
                    .format("%F %R"),
            )
        });
}
