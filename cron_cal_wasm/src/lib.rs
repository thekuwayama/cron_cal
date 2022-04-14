use chrono::{DateTime, NaiveDateTime, Utc};
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
    let date = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(date, 0), Utc);
    let mut input = input.as_bytes();

    match parse(&mut input, date, days as usize) {
        Ok(cal) => Ok(JsValue::from_serde(&JsCronSchedules {
            schedules: format_unix_timestamp(&cal, date)
                .iter()
                .map(|p| JsCronSchedule {
                    start: p.start,
                    end: p.end,
                })
                .collect(),
        })
        .unwrap()),
        _ => Err(JsValue::from("cron format error")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::prelude::*;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_parse_cron_cal() {
        let date = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0).timestamp();
        let result = parse_cron_cal("\"30 9,12,15 1,15 May-Aug Mon,Wed,Fri *\",5", date, 1);
        assert!(result.is_ok());

        let expected = JsValue::from_serde(&JsCronSchedules {
            schedules: vec![
                // -> 2018-06-01 09:30:00 UTC
                JsCronSchedule {
                    start: Utc.ymd(2018, 6, 1).and_hms(9, 30, 0).timestamp(),
                    end: Utc.ymd(2018, 6, 1).and_hms(9, 35, 0).timestamp(),
                },
                // -> 2018-06-01 12:30:00 UTC
                JsCronSchedule {
                    start: Utc.ymd(2018, 6, 1).and_hms(12, 30, 0).timestamp(),
                    end: Utc.ymd(2018, 6, 1).and_hms(12, 35, 0).timestamp(),
                },
                // -> 2018-06-01 15:30:00 UTC
                JsCronSchedule {
                    start: Utc.ymd(2018, 6, 1).and_hms(15, 30, 0).timestamp(),
                    end: Utc.ymd(2018, 6, 1).and_hms(15, 35, 0).timestamp(),
                },
            ],
        })
        .unwrap()
        .into_serde()
        .unwrap();
        let result: JsCronSchedules = result.unwrap().into_serde().unwrap();
        assert_eq!(result, expected);
    }
}
