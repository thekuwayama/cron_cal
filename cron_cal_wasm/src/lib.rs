use chrono::{NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use cron_cal::format::format_unix_timestamp;
use cron_cal::parse::parse;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct JsCronSchedule {
    start: i64,
    end: i64,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct JsCronSchedules {
    schedules: Vec<JsCronSchedule>,
}

#[wasm_bindgen]
pub fn parse_cron_cal(input: &str, date: i64, days: i32) -> Result<JsValue, JsValue> {
    let date = Utc.from_utc_datetime(match &NaiveDateTime::from_timestamp_opt(date, 0) {
        Some(date) => date,
        _ => return Err(JsValue::from("date format error")),
    });
    let mut input = input.as_bytes();

    match parse(&mut input, date, days as usize) {
        Ok(cal) => Ok(serde_wasm_bindgen::to_value(&JsCronSchedules {
            schedules: format_unix_timestamp(&cal, date)
                .iter()
                .map(|p| JsCronSchedule {
                    start: p.start,
                    end: p.end,
                })
                .collect(),
        })?),
        _ => Err(JsValue::from("cron format error")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_parse_cron_cal() {
        let date = Utc
            .with_ymd_and_hms(2018, 6, 1, 0, 0, 0)
            .unwrap()
            .timestamp();
        let result = parse_cron_cal("\"30 9,12,15 1,15 May-Aug Mon,Wed,Fri *\",5", date, 1);
        assert!(result.is_ok());

        let expected = JsCronSchedules {
            schedules: vec![
                // -> 2018-06-01 09:30:00 UTC
                JsCronSchedule {
                    start: Utc
                        .with_ymd_and_hms(2018, 6, 1, 9, 30, 0)
                        .unwrap()
                        .timestamp(),
                    end: Utc
                        .with_ymd_and_hms(2018, 6, 1, 9, 35, 0)
                        .unwrap()
                        .timestamp(),
                },
                // -> 2018-06-01 12:30:00 UTC
                JsCronSchedule {
                    start: Utc
                        .with_ymd_and_hms(2018, 6, 1, 12, 30, 0)
                        .unwrap()
                        .timestamp(),
                    end: Utc
                        .with_ymd_and_hms(2018, 6, 1, 12, 35, 0)
                        .unwrap()
                        .timestamp(),
                },
                // -> 2018-06-01 15:30:00 UTC
                JsCronSchedule {
                    start: Utc
                        .with_ymd_and_hms(2018, 6, 1, 15, 30, 0)
                        .unwrap()
                        .timestamp(),
                    end: Utc
                        .with_ymd_and_hms(2018, 6, 1, 15, 35, 0)
                        .unwrap()
                        .timestamp(),
                },
            ],
        };
        let result: JsCronSchedules = serde_wasm_bindgen::from_value(result.unwrap()).unwrap();
        assert_eq!(result, expected);
    }
}
