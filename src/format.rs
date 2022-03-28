use chrono::offset::Utc;
use chrono::{DateTime, Duration};

use crate::r#type::CronCalender;

fn format_cal1(cal: &CronCalender, scale: usize, start: DateTime<Utc>) -> String {
    cal.chunks(scale)
        .map(|b| b.any())
        .enumerate()
        .filter_map(|(i, b)| {
            if b {
                let start = start + Duration::minutes((i * scale) as i64);
                let end = start + Duration::minutes(scale as i64);
                Some(format!("{} ~ {}", start, end))
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub(crate) fn format_cal(cal: &Vec<CronCalender>, scale: usize, start: DateTime<Utc>) -> String {
    cal.into_iter()
        .enumerate()
        .map(|(i, c)| format_cal1(&c, scale, start + Duration::days(i as i64)))
        .collect::<Vec<String>>()
        .join("\n")
}

fn format_cal_spare1(cal: &CronCalender, scale: usize, start: DateTime<Utc>) -> String {
    cal.chunks(scale)
        .map(|b| b.not_any())
        .enumerate()
        .filter_map(|(i, b)| {
            if b {
                let start = start + Duration::minutes((i * scale) as i64);
                let end = start + Duration::minutes(scale as i64);
                Some(format!("{} ~ {}", start, end))
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub(crate) fn format_cal_spare(
    cal: &Vec<CronCalender>,
    scale: usize,
    start: DateTime<Utc>,
) -> String {
    cal.into_iter()
        .enumerate()
        .map(|(i, c)| format_cal_spare1(&c, scale, start + Duration::days(i as i64)))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::prelude::*;

    const QUARTER: usize = 15;
    const HALF: usize = 30;
    const HOUR: usize = 60;

    #[test]
    fn test_format_cal() {
        let mut cal = CronCalender::default();
        // -> 2018-06-01 09:30:00 UTC
        (570..=575).for_each(|i| cal.set(i, true));
        // -> 2018-06-01 12:30:00 UTC
        (750..=755).for_each(|i| cal.set(i, true));
        // -> 2018-06-01 15:30:00 UTC
        (930..=935).for_each(|i| cal.set(i, true));
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);

        // 15
        let result = format_cal(&vec![cal], QUARTER, target);
        assert_eq!(result, "2018-06-01 09:30:00 UTC ~ 2018-06-01 09:45:00 UTC\n2018-06-01 12:30:00 UTC ~ 2018-06-01 12:45:00 UTC\n2018-06-01 15:30:00 UTC ~ 2018-06-01 15:45:00 UTC");
        // 30
        let result = format_cal(&vec![cal], HALF, target);
        assert_eq!(result, "2018-06-01 09:30:00 UTC ~ 2018-06-01 10:00:00 UTC\n2018-06-01 12:30:00 UTC ~ 2018-06-01 13:00:00 UTC\n2018-06-01 15:30:00 UTC ~ 2018-06-01 16:00:00 UTC");
        // 60
        let result = format_cal(&vec![cal], HOUR, target);
        assert_eq!(result, "2018-06-01 09:00:00 UTC ~ 2018-06-01 10:00:00 UTC\n2018-06-01 12:00:00 UTC ~ 2018-06-01 13:00:00 UTC\n2018-06-01 15:00:00 UTC ~ 2018-06-01 16:00:00 UTC");
    }

    #[test]
    fn test_format_cal_spare() {
        let mut cal = CronCalender::default();
        // -> 2018-06-01 09:30:00 UTC
        (0..570).for_each(|i| cal.set(i, true));
        // -> 2018-06-01 12:30:00 UTC
        (750..1440).for_each(|i| cal.set(i, true));
        let target = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);

        // 15
        let result = format_cal_spare(&vec![cal], QUARTER, target);
        assert_eq!(result, "2018-06-01 09:30:00 UTC ~ 2018-06-01 09:45:00 UTC\n2018-06-01 09:45:00 UTC ~ 2018-06-01 10:00:00 UTC\n2018-06-01 10:00:00 UTC ~ 2018-06-01 10:15:00 UTC\n2018-06-01 10:15:00 UTC ~ 2018-06-01 10:30:00 UTC\n2018-06-01 10:30:00 UTC ~ 2018-06-01 10:45:00 UTC\n2018-06-01 10:45:00 UTC ~ 2018-06-01 11:00:00 UTC\n2018-06-01 11:00:00 UTC ~ 2018-06-01 11:15:00 UTC\n2018-06-01 11:15:00 UTC ~ 2018-06-01 11:30:00 UTC\n2018-06-01 11:30:00 UTC ~ 2018-06-01 11:45:00 UTC\n2018-06-01 11:45:00 UTC ~ 2018-06-01 12:00:00 UTC\n2018-06-01 12:00:00 UTC ~ 2018-06-01 12:15:00 UTC\n2018-06-01 12:15:00 UTC ~ 2018-06-01 12:30:00 UTC");
        // 30
        let result = format_cal_spare(&vec![cal], HALF, target);
        assert_eq!(result, "2018-06-01 09:30:00 UTC ~ 2018-06-01 10:00:00 UTC\n2018-06-01 10:00:00 UTC ~ 2018-06-01 10:30:00 UTC\n2018-06-01 10:30:00 UTC ~ 2018-06-01 11:00:00 UTC\n2018-06-01 11:00:00 UTC ~ 2018-06-01 11:30:00 UTC\n2018-06-01 11:30:00 UTC ~ 2018-06-01 12:00:00 UTC\n2018-06-01 12:00:00 UTC ~ 2018-06-01 12:30:00 UTC");
        // 60
        let result = format_cal_spare(&vec![cal], HOUR, target);
        assert_eq!(result, "2018-06-01 10:00:00 UTC ~ 2018-06-01 11:00:00 UTC\n2018-06-01 11:00:00 UTC ~ 2018-06-01 12:00:00 UTC");
    }
}
