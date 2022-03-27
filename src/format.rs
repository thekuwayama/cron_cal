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
