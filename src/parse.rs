use std::io::BufRead;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use chrono::offset::Utc;
use chrono::{DateTime, Duration};
use cron::Schedule;

use crate::r#type::{CronCalender, MINUTES_OF_DAY, MINUTES_OF_HOUR};

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

pub(crate) fn parse<R: BufRead>(
    reader: &mut R,
    time_required: usize,
    target: DateTime<Utc>,
) -> Result<CronCalender> {
    let mut result = CronCalender::default();
    let next_day = target + Duration::days(1);

    do_parse(reader)?.into_iter().for_each(|s| {
        // supports jobs that starts the day before
        let mut iter = s.after(&(target - Duration::minutes(time_required as i64)));
        for start in iter.by_ref() {
            if start > next_day {
                break;
            }

            let start = (start.timestamp() - target.timestamp()) / MINUTES_OF_HOUR as i64;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    use chrono::prelude::*;

    const TIME_REQUIRED: usize = 5;

    #[test]
    fn test_parse() {
        let mut reader = BufReader::new("0 30 9,12,15 1,15 May-Aug Mon,Wed,Fri *".as_bytes());
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);
        let result = parse(&mut reader, TIME_REQUIRED, target);
        assert!(result.is_ok());

        let mut expected = CronCalender::default();
        // -> 2018-06-01 09:30:00 UTC
        (570..=575).for_each(|i| expected.set(i, true));
        // -> 2018-06-01 12:30:00 UTC
        (750..=755).for_each(|i| expected.set(i, true));
        // -> 2018-06-01 15:30:00 UTC
        (930..=935).for_each(|i| expected.set(i, true));
        assert_eq!(result.unwrap(), expected);
    }
}
