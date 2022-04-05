use chrono::offset::Utc;
use chrono::{DateTime, Duration};

use crate::r#type::CronCalender;

fn format_rfc3339_rounding_1(
    cal: &CronCalender,
    scale: usize,
    start: DateTime<Utc>,
) -> Vec<(String, String)> {
    cal.chunks(scale)
        .map(|b| b.any())
        .enumerate()
        .filter_map(|(i, b)| {
            if b {
                let start = start + Duration::minutes((i * scale) as i64);
                let end = start + Duration::minutes(scale as i64);
                Some((start.to_rfc3339(), end.to_rfc3339()))
            } else {
                None
            }
        })
        .collect()
}

pub fn format_rfc3339_rounding(
    cal: &[CronCalender],
    scale: usize,
    start: DateTime<Utc>,
) -> Vec<(String, String)> {
    cal.iter()
        .enumerate()
        .flat_map(|(i, c)| format_rfc3339_rounding_1(c, scale, start + Duration::days(i as i64)))
        .collect()
}

fn format_rfc3339_rounding_spare_1(
    cal: &CronCalender,
    scale: usize,
    start: DateTime<Utc>,
) -> Vec<(String, String)> {
    cal.chunks(scale)
        .map(|b| b.not_any())
        .enumerate()
        .filter_map(|(i, b)| {
            if b {
                let start = start + Duration::minutes((i * scale) as i64);
                let end = start + Duration::minutes(scale as i64);
                Some((start.to_rfc3339(), end.to_rfc3339()))
            } else {
                None
            }
        })
        .collect()
}

pub fn format_rfc3339_rounding_spare(
    cal: &[CronCalender],
    scale: usize,
    start: DateTime<Utc>,
) -> Vec<(String, String)> {
    cal.iter()
        .enumerate()
        .flat_map(|(i, c)| {
            format_rfc3339_rounding_spare_1(c, scale, start + Duration::days(i as i64))
        })
        .collect()
}

fn format_rfc3339_1(cal: &CronCalender, start: DateTime<Utc>) -> Vec<(String, String)> {
    let mut res = cal.iter().enumerate().fold(vec![], |mut acc, (i, b)| {
        if i == 0 && *b {
            let start = start + Duration::minutes(i as i64);
            acc.push((start.to_rfc3339(), "".to_owned()));
            acc
        } else if i == 0 {
            acc
        } else if *b && acc.is_empty() {
            let start = start + Duration::minutes(i as i64);
            acc.push((start.to_rfc3339(), "".to_owned()));
            acc
        } else if *b && acc.last().map(|p| p.1.is_empty()).unwrap_or(true) {
            acc
        } else if *b && acc.last().map(|p| !p.1.is_empty()).unwrap_or(true) {
            let start = start + Duration::minutes(i as i64);
            acc.push((start.to_rfc3339(), "".to_owned()));
            acc
        } else if !*b && acc.is_empty() {
            acc
        } else if !*b && acc.last().map(|p| p.1.is_empty()).unwrap_or(true) {
            let tmp = acc.pop();
            let end = start + Duration::minutes((i - 1) as i64);
            acc.push((
                tmp.map(|p| p.0).unwrap_or_else(|| "".to_owned()),
                end.to_rfc3339(),
            ));
            acc
        } else {
            acc
        }
    });

    if let Some(last) = res.last_mut() {
        if last.1.is_empty() {
            *last = (last.0.clone(), (start + Duration::days(1)).to_rfc3339());
        }
    }

    res
}

pub fn format_rfc3339(cal: &[CronCalender], start: DateTime<Utc>) -> Vec<(String, String)> {
    cal.iter()
        .enumerate()
        .flat_map(|(i, c)| format_rfc3339_1(c, start + Duration::days(i as i64)))
        .collect()
}

fn format_rfc3339_spare_1(cal: &CronCalender, start: DateTime<Utc>) -> Vec<(String, String)> {
    let mut res = cal.iter().enumerate().fold(vec![], |mut acc, (i, b)| {
        if i == 0 && !*b {
            let start = start + Duration::minutes(i as i64);
            acc.push((start.to_rfc3339(), "".to_owned()));
            acc
        } else if i == 0 {
            acc
        } else if !*b && acc.is_empty() {
            let start = start + Duration::minutes(i as i64);
            acc.push((start.to_rfc3339(), "".to_owned()));
            acc
        } else if !*b && acc.last().map(|p| p.1.is_empty()).unwrap_or(true) {
            acc
        } else if !*b && acc.last().map(|p| !p.1.is_empty()).unwrap_or(true) {
            let start = start + Duration::minutes(i as i64);
            acc.push((start.to_rfc3339(), "".to_owned()));
            acc
        } else if *b && acc.is_empty() {
            acc
        } else if *b && acc.last().map(|p| p.1.is_empty()).unwrap_or(true) {
            let tmp = acc.pop();
            let end = start + Duration::minutes(i as i64);
            acc.push((
                tmp.map(|p| p.0).unwrap_or_else(|| "".to_owned()),
                end.to_rfc3339(),
            ));
            acc
        } else {
            acc
        }
    });

    if let Some(last) = res.last_mut() {
        if last.1.is_empty() {
            *last = (last.0.clone(), (start + Duration::days(1)).to_rfc3339());
        }
    }

    res
}

pub fn format_rfc3339_spare(cal: &[CronCalender], start: DateTime<Utc>) -> Vec<(String, String)> {
    cal.iter()
        .enumerate()
        .flat_map(|(i, c)| format_rfc3339_spare_1(c, start + Duration::days(i as i64)))
        .collect()
}

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

    const QUARTER: usize = 15;
    const HALF: usize = 30;
    const HOUR: usize = 60;

    #[test]
    fn test_format_rfc3339_rounding() {
        let mut cal = CronCalender::default();
        // -> 2018-06-01T09:30:00+00:00
        (570..=575).for_each(|i| cal.set(i, true));
        // -> 2018-06-01T12:30:00+00:00
        (750..=755).for_each(|i| cal.set(i, true));
        // -> 2018-06-01T15:30:00+00:00
        (930..=935).for_each(|i| cal.set(i, true));
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);

        // 15
        let result = format_rfc3339_rounding(&vec![cal], QUARTER, target);
        assert_eq!(
            result
                .iter()
                .map(|p| (p.0.as_str(), p.1.as_str()))
                .collect::<Vec<(&str, &str)>>(),
            vec![
                ("2018-06-01T09:30:00+00:00", "2018-06-01T09:45:00+00:00"),
                ("2018-06-01T12:30:00+00:00", "2018-06-01T12:45:00+00:00"),
                ("2018-06-01T15:30:00+00:00", "2018-06-01T15:45:00+00:00")
            ]
        );
        // 30
        let result = format_rfc3339_rounding(&vec![cal], HALF, target);
        assert_eq!(
            result
                .iter()
                .map(|p| (p.0.as_str(), p.1.as_str()))
                .collect::<Vec<(&str, &str)>>(),
            vec![
                ("2018-06-01T09:30:00+00:00", "2018-06-01T10:00:00+00:00"),
                ("2018-06-01T12:30:00+00:00", "2018-06-01T13:00:00+00:00"),
                ("2018-06-01T15:30:00+00:00", "2018-06-01T16:00:00+00:00")
            ]
        );
        // 60
        let result = format_rfc3339_rounding(&vec![cal], HOUR, target);
        assert_eq!(
            result
                .iter()
                .map(|p| (p.0.as_str(), p.1.as_str()))
                .collect::<Vec<(&str, &str)>>(),
            vec![
                ("2018-06-01T09:00:00+00:00", "2018-06-01T10:00:00+00:00"),
                ("2018-06-01T12:00:00+00:00", "2018-06-01T13:00:00+00:00"),
                ("2018-06-01T15:00:00+00:00", "2018-06-01T16:00:00+00:00")
            ]
        );
    }

    #[test]
    fn test_format_rfc3339_rounding_spare() {
        let mut cal = CronCalender::default();
        // -> 2018-06-01T09:30:00+00:00
        (0..570).for_each(|i| cal.set(i, true));
        // -> 2018-06-01T12:30:00+00:00
        (750..1440).for_each(|i| cal.set(i, true));
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);

        // 15
        let result = format_rfc3339_rounding_spare(&vec![cal], QUARTER, target);
        assert_eq!(
            result
                .iter()
                .map(|p| (p.0.as_str(), p.1.as_str()))
                .collect::<Vec<(&str, &str)>>(),
            vec![
                ("2018-06-01T09:30:00+00:00", "2018-06-01T09:45:00+00:00"),
                ("2018-06-01T09:45:00+00:00", "2018-06-01T10:00:00+00:00"),
                ("2018-06-01T10:00:00+00:00", "2018-06-01T10:15:00+00:00"),
                ("2018-06-01T10:15:00+00:00", "2018-06-01T10:30:00+00:00"),
                ("2018-06-01T10:30:00+00:00", "2018-06-01T10:45:00+00:00"),
                ("2018-06-01T10:45:00+00:00", "2018-06-01T11:00:00+00:00"),
                ("2018-06-01T11:00:00+00:00", "2018-06-01T11:15:00+00:00"),
                ("2018-06-01T11:15:00+00:00", "2018-06-01T11:30:00+00:00"),
                ("2018-06-01T11:30:00+00:00", "2018-06-01T11:45:00+00:00"),
                ("2018-06-01T11:45:00+00:00", "2018-06-01T12:00:00+00:00"),
                ("2018-06-01T12:00:00+00:00", "2018-06-01T12:15:00+00:00"),
                ("2018-06-01T12:15:00+00:00", "2018-06-01T12:30:00+00:00")
            ]
        );
        // 30
        let result = format_rfc3339_rounding_spare(&vec![cal], HALF, target);
        assert_eq!(
            result
                .iter()
                .map(|p| (p.0.as_str(), p.1.as_str()))
                .collect::<Vec<(&str, &str)>>(),
            vec![
                ("2018-06-01T09:30:00+00:00", "2018-06-01T10:00:00+00:00"),
                ("2018-06-01T10:00:00+00:00", "2018-06-01T10:30:00+00:00"),
                ("2018-06-01T10:30:00+00:00", "2018-06-01T11:00:00+00:00"),
                ("2018-06-01T11:00:00+00:00", "2018-06-01T11:30:00+00:00"),
                ("2018-06-01T11:30:00+00:00", "2018-06-01T12:00:00+00:00"),
                ("2018-06-01T12:00:00+00:00", "2018-06-01T12:30:00+00:00")
            ]
        );
        // 60
        let result = format_rfc3339_rounding_spare(&vec![cal], HOUR, target);
        assert_eq!(
            result
                .iter()
                .map(|p| (p.0.as_str(), p.1.as_str()))
                .collect::<Vec<(&str, &str)>>(),
            vec![
                ("2018-06-01T10:00:00+00:00", "2018-06-01T11:00:00+00:00"),
                ("2018-06-01T11:00:00+00:00", "2018-06-01T12:00:00+00:00")
            ]
        );
    }

    #[test]
    fn test_format_rfc3339() {
        let mut cal = CronCalender::default();
        // -> 2018-06-01T09:30:00+00:00
        (570..=575).for_each(|i| cal.set(i, true));
        // -> 2018-06-01T12:30:00+00:00
        (750..=755).for_each(|i| cal.set(i, true));
        // -> 2018-06-01T15:30:00+00:00
        (930..=935).for_each(|i| cal.set(i, true));
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);

        let result = format_rfc3339(&vec![cal], target);
        assert_eq!(
            result
                .iter()
                .map(|p| (p.0.as_str(), p.1.as_str()))
                .collect::<Vec<(&str, &str)>>(),
            vec![
                ("2018-06-01T09:30:00+00:00", "2018-06-01T09:35:00+00:00"),
                ("2018-06-01T12:30:00+00:00", "2018-06-01T12:35:00+00:00"),
                ("2018-06-01T15:30:00+00:00", "2018-06-01T15:35:00+00:00")
            ]
        );

        let mut cal = CronCalender::default();
        // -> all day
        (0..1440).for_each(|i| cal.set(i, true));
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);

        let result = format_rfc3339(&vec![cal], target);
        assert_eq!(
            result
                .iter()
                .map(|p| (p.0.as_str(), p.1.as_str()))
                .collect::<Vec<(&str, &str)>>(),
            vec![("2018-06-01T00:00:00+00:00", "2018-06-02T00:00:00+00:00")]
        );
    }

    #[test]
    fn test_format_rfc3339_spare() {
        let mut cal = CronCalender::default();
        // -> 2018-06-01T09:30:00+00:00
        (0..570).for_each(|i| cal.set(i, true));
        // -> 2018-06-01T12:30:00+00:00
        (750..1440).for_each(|i| cal.set(i, true));
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);

        let result = format_rfc3339_spare(&vec![cal], target);
        assert_eq!(
            result
                .iter()
                .map(|p| (p.0.as_str(), p.1.as_str()))
                .collect::<Vec<(&str, &str)>>(),
            vec![("2018-06-01T09:30:00+00:00", "2018-06-01T12:30:00+00:00")]
        );

        let cal = CronCalender::default();
        // -> not scheduled
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);

        let result = format_rfc3339_spare(&vec![cal], target);
        assert_eq!(
            result
                .iter()
                .map(|p| (p.0.as_str(), p.1.as_str()))
                .collect::<Vec<(&str, &str)>>(),
            vec![("2018-06-01T00:00:00+00:00", "2018-06-02T00:00:00+00:00")]
        );
    }

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
