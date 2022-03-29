use std::io::BufRead;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use chrono::offset::Utc;
use chrono::{DateTime, Duration};
use cron::Schedule;
use csv::ReaderBuilder as CsvReaderBuilder;
use serde::Deserialize;

use crate::r#type::{CronCalender, MINUTES_OF_DAY, MINUTES_OF_HOUR};

struct CronSchedule {
    schedule: Schedule,
    time_required: usize,
}

fn cron_schedule(s: &str) -> Result<Vec<CronSchedule>> {
    #[derive(Debug, Deserialize)]
    struct Record {
        schedule: String,
        time_required: usize,
    }

    CsvReaderBuilder::new()
        .has_headers(false)
        .from_reader(s.as_bytes())
        .deserialize::<Record>()
        .map(|r| {
            let record: Record = r?;
            Ok(CronSchedule {
                schedule: Schedule::from_str(&record.schedule)?,
                time_required: record.time_required,
            })
        })
        .collect()
}

fn do_parse<R: BufRead>(reader: &mut R) -> Result<Vec<CronSchedule>> {
    let (vec, err): (Vec<_>, Vec<_>) = reader
        .lines()
        .filter_map(Result::ok)
        .map(|s| cron_schedule(&s))
        .partition(Result::is_ok);
    if !err.is_empty() {
        return Err(anyhow!("Failed to parse cron schedule"));
    }

    Ok(vec.into_iter().flat_map(Result::unwrap).collect())
}

fn parse1(schedule: &[CronSchedule], target: DateTime<Utc>) -> Result<CronCalender> {
    let next_day = target + Duration::days(1);
    let result = schedule.iter().fold(CronCalender::default(), |mut r, c| {
        // supports jobs that starts the day before
        c.schedule
            .after(&(target - Duration::minutes(c.time_required as i64)))
            .take_while(|start| start < &next_day)
            .for_each(|start| {
                let start = (start.timestamp() - target.timestamp()) / MINUTES_OF_HOUR as i64;
                let end = start + c.time_required as i64;
                (start..=end).for_each(|i| {
                    if i < MINUTES_OF_DAY as i64 {
                        r.set(i as usize, true);
                    }
                });
            });

        r
    });

    Ok(result)
}

pub(crate) fn parse<R: BufRead>(
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
        let mut reader = BufReader::new("\"0 30 9,12,15 1,15 May-Aug Mon,Wed,Fri *\",5".as_bytes());
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
