use std::io::BufRead;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Utc};
use cron::Schedule;
use csv::ReaderBuilder as CsvReaderBuilder;
use csv::Trim;
use serde::Deserialize;

use crate::r#type::{CronCalender, MINUTES_OF_DAY, SECONDS_OF_MINUTE};

struct CronRecord {
    schedule: Schedule,
    time_required: usize,
}

fn cron_schedule(s: &str) -> Result<Vec<CronRecord>> {
    #[derive(Debug, Deserialize)]
    struct Record {
        schedule: String,
        time_required: usize,
    }

    CsvReaderBuilder::new()
        .has_headers(false)
        .trim(Trim::Fields)
        .from_reader(s.as_bytes())
        .deserialize::<Record>()
        .map(|r| {
            let record: Record = r?;
            Ok(CronRecord {
                schedule: Schedule::from_str(&format!("0 {}", record.schedule))?,
                time_required: record.time_required,
            })
        })
        .collect()
}

fn do_parse<R: BufRead>(reader: &mut R) -> Result<Vec<CronRecord>> {
    let (vec, err): (Vec<_>, Vec<_>) = reader
        .lines()
        .filter_map(Result::ok)
        .map(|s| cron_schedule(s.trim()))
        .partition(Result::is_ok);
    if !err.is_empty() {
        return Err(anyhow!("Failed to parse cron schedule"));
    }

    Ok(vec.into_iter().flat_map(Result::unwrap).collect())
}

fn parse1(schedule: &[CronRecord], target: DateTime<Utc>) -> Result<CronCalender> {
    let next_day = target + Duration::days(1);
    let result = schedule.iter().fold(CronCalender::default(), |mut r, c| {
        // supports jobs that starts the day before
        c.schedule
            .after(&(target - Duration::minutes(c.time_required as i64)))
            .take_while(|start| start < &next_day)
            .for_each(|start| {
                let start = (start.timestamp() - target.timestamp()) / SECONDS_OF_MINUTE as i64;
                let end = start + c.time_required as i64;
                (start..=end).for_each(|i| {
                    if i >= 0 && i < MINUTES_OF_DAY as i64 {
                        r.set(i as usize, true);
                    }
                });
            });

        r
    });

    Ok(result)
}

pub fn parse<R: BufRead>(
    reader: &mut R,
    target: DateTime<Utc>,
    period: usize,
) -> Result<Vec<CronCalender>> {
    let schedule = do_parse(reader)?;

    (0..period)
        .map(|i| parse1(&schedule, target + Duration::days(i as i64)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    use chrono::prelude::*;

    #[test]
    fn test_parse() {
        let mut reader = BufReader::new("\"30 9,12,15 1,15 May-Aug Mon,Wed,Fri *\",5".as_bytes());
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);
        let result = parse(&mut reader, target, 1);
        assert!(result.is_ok());

        let mut expected = CronCalender::default();
        // -> 2018-06-01 09:30:00 UTC
        (570..=575).for_each(|i| expected.set(i, true));
        // -> 2018-06-01 12:30:00 UTC
        (750..=755).for_each(|i| expected.set(i, true));
        // -> 2018-06-01 15:30:00 UTC
        (930..=935).for_each(|i| expected.set(i, true));
        assert_eq!(result.unwrap(), vec![expected]);
    }

    #[test]
    fn test_parse_multiline_input() {
        let mut reader = BufReader::new(
            "\"30 9 * * * *\",5\n\"30 12 * * * *\",5\n\"30 15 * * * *\",5".as_bytes(),
        );
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);
        let result = parse(&mut reader, target, 1);
        assert!(result.is_ok());

        let mut expected = CronCalender::default();
        // -> 2018-06-01 09:30:00 UTC
        (570..=575).for_each(|i| expected.set(i, true));
        // -> 2018-06-01 12:30:00 UTC
        (750..=755).for_each(|i| expected.set(i, true));
        // -> 2018-06-01 15:30:00 UTC
        (930..=935).for_each(|i| expected.set(i, true));
        assert_eq!(result.unwrap(), vec![expected]);
    }

    #[test]
    fn test_parse_every_minute() {
        let mut reader = BufReader::new("\"* * * * * *\",1".as_bytes());
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);
        let result = parse(&mut reader, target, 1);
        assert!(result.is_ok());

        let mut expected = CronCalender::default();
        (0..1440).for_each(|i| expected.set(i, true));
        assert_eq!(result.unwrap(), vec![expected]);
    }

    #[test]
    fn test_parse_all_day() {
        let mut reader = BufReader::new("\"0 * * * * *\",1440".as_bytes());
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);
        let result = parse(&mut reader, target, 1);
        assert!(result.is_ok());

        let mut expected = CronCalender::default();
        (0..1440).for_each(|i| expected.set(i, true));
        assert_eq!(result.unwrap(), vec![expected]);
    }

    #[test]
    fn test_parse_omit_year() {
        let mut reader = BufReader::new("\"30 9,12,15 1,15 May-Aug Mon,Wed,Fri\",5".as_bytes());
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);
        let result = parse(&mut reader, target, 1);
        assert!(result.is_ok());

        let mut expected = CronCalender::default();
        // -> 2018-06-01 09:30:00 UTC
        (570..=575).for_each(|i| expected.set(i, true));
        // -> 2018-06-01 12:30:00 UTC
        (750..=755).for_each(|i| expected.set(i, true));
        // -> 2018-06-01 15:30:00 UTC
        (930..=935).for_each(|i| expected.set(i, true));
        assert_eq!(result.unwrap(), vec![expected]);
    }
}
