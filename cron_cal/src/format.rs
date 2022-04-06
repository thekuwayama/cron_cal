use chrono::offset::Utc;
use chrono::{DateTime, Duration};

use crate::r#type::CronCalender;

fn format_unix_timestamp_1(cal: &CronCalender, start: DateTime<Utc>) -> Vec<(i64, i64)> {
    let mut res = cal.iter().enumerate().fold(vec![], |mut acc, (i, b)| {
        if i == 0 && *b {
            let start = start + Duration::minutes(i as i64);
            acc.push((start.timestamp(), -1));
            acc
        } else if i == 0 {
            acc
        } else if *b && acc.is_empty() {
            let start = start + Duration::minutes(i as i64);
            acc.push((start.timestamp(), -1));
            acc
        } else if *b && acc.last().map(|p| p.1).unwrap_or(-1) < 0 {
            acc
        } else if *b && acc.last().map(|p| p.1).unwrap_or(0) >= 0 {
            let start = start + Duration::minutes(i as i64);
            acc.push((start.timestamp(), -1));
            acc
        } else if !*b && acc.is_empty() {
            acc
        } else if !*b && acc.last().map(|p| p.1).unwrap_or(-1) < 0 {
            let tmp = acc.pop();
            let end = start + Duration::minutes((i - 1) as i64);
            acc.push((tmp.map(|p| p.0).unwrap_or(0), end.timestamp()));
            acc
        } else {
            acc
        }
    });

    if let Some(last) = res.last_mut() {
        if last.1 < 0 {
            *last = (last.0, (start + Duration::days(1)).timestamp());
        }
    }

    res
}

#[allow(dead_code)]
pub fn format_unix_timestamp(cal: &[CronCalender], start: DateTime<Utc>) -> Vec<(i64, i64)> {
    cal.iter()
        .enumerate()
        .flat_map(|(i, c)| format_unix_timestamp_1(c, start + Duration::days(i as i64)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::prelude::*;

    #[test]
    fn test_format_unix_timestamp() {
        let mut cal = CronCalender::default();
        // -> 2018-06-01T09:30:00+00:00
        (570..=575).for_each(|i| cal.set(i, true));
        // -> 2018-06-01T12:30:00+00:00
        (750..=755).for_each(|i| cal.set(i, true));
        // -> 2018-06-01T15:30:00+00:00
        (930..=935).for_each(|i| cal.set(i, true));
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);

        let result = format_unix_timestamp(&vec![cal], target);
        assert_eq!(
            result
                .iter()
                .map(|p| (p.0, p.1))
                .collect::<Vec<(i64, i64)>>(),
            vec![
                (1527845400, 1527845700),
                (1527856200, 1527856500),
                (1527867000, 1527867300)
            ]
        );

        let mut cal = CronCalender::default();
        // -> all day
        (0..1440).for_each(|i| cal.set(i, true));
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);

        let result = format_unix_timestamp(&vec![cal], target);
        assert_eq!(
            result
                .iter()
                .map(|p| (p.0, p.1))
                .collect::<Vec<(i64, i64)>>(),
            vec![(1527811200, 1527897600)]
        );
    }
}
