use chrono::offset::Utc;
use chrono::{DateTime, Duration};

use crate::r#type::CronCalender;

pub(crate) fn format_cal(cal: &CronCalender, scale: usize, start: DateTime<Utc>) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::prelude::*;

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
        let result = format_cal(&cal, 30, target);
        assert_eq!(result, "2018-06-01 09:30:00 UTC ~ 2018-06-01 10:00:00 UTC\n2018-06-01 12:30:00 UTC ~ 2018-06-01 13:00:00 UTC\n2018-06-01 15:30:00 UTC ~ 2018-06-01 16:00:00 UTC");
    }
}
