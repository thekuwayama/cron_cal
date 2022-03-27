use std::io::BufRead;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use chrono::{Duration, Utc};
use cron::Schedule;

const MINUTES_OF_HOUR: usize = 60;
const MINUTES_OF_DAY: usize = 24 * MINUTES_OF_HOUR;

pub(crate) type CronCalender = BitArray<[u8; MINUTES_OF_DAY / 8]>;

fn do_parse<R: BufRead>(reader: &mut R) -> Result<Vec<Schedule>> {
    let (vec, err): (Vec<_>, Vec<_>) = reader
        .lines()
        .filter_map(Result::ok)
        .map(|s| Schedule::from_str(&s))
        .partition(Result::is_ok);
    if !err.is_empty() {
        return Err(anyhow!("Failed to parse cron schedule"));
    }

    Ok(vec.into_iter().map(Result::unwrap).collect())
}

pub(crate) fn parse<R: BufRead>(reader: &mut R, time_required: usize) -> Result<CronCalender> {
    let mut result = BitArray::<[u8; MINUTES_OF_DAY / 8]>::default();
    let today = Utc::today().and_hms(0, 0, 0);
    let tomorrow = today + Duration::days(1);

    do_parse(reader)?.into_iter().for_each(|s| {
        // supports jobs that starts the day before
        let mut iter = s.after(&(today - Duration::minutes(time_required as i64)));
        for start in iter.by_ref() {
            if start > tomorrow {
                break;
            }

            let start = (start.timestamp() - today.timestamp()) / MINUTES_OF_HOUR as i64;
            let end = start + time_required as i64;
            (start..=end).for_each(|i| {
                if i < MINUTES_OF_DAY as i64 {
                    result.set(i as usize, true);
                }
            });
        }
    });

    Ok(result)
}
