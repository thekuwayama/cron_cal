use chrono::{DateTime, NaiveDateTime, Utc};
use cron_cal::format::format_unix_timestamp;
use cron_cal::parse::parse;
use js_sys::{Array, BigInt, BigInt64Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn parse_cron_cal(input: &str, date: i64, days: i32) -> Result<BigInt64Array, JsValue> {
    let date = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(date, 0), Utc);
    let mut input = input.as_bytes();

    match parse(&mut input, date, days as usize) {
        Ok(cal) => Ok(BigInt64Array::new(&JsValue::from(
            format_unix_timestamp(&cal, date)
                .iter()
                .flat_map(|p| vec![p.start, p.end])
                .map(BigInt::from)
                .collect::<Array>(),
        ))),
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

        let result = result.unwrap();
        // -> 2018-06-01 09:30:00 UTC
        assert!(result.has_own_property(&JsValue::from(
            Utc.ymd(2018, 6, 1).and_hms(9, 30, 0).timestamp()
        )));
        assert!(result.has_own_property(&JsValue::from(
            Utc.ymd(2018, 6, 1).and_hms(9, 35, 0).timestamp()
        )));
        // -> 2018-06-01 12:30:00 UTC
        assert!(result.has_own_property(&JsValue::from(
            Utc.ymd(2018, 6, 1).and_hms(12, 30, 0).timestamp()
        )));
        assert!(result.has_own_property(&JsValue::from(
            Utc.ymd(2018, 6, 1).and_hms(12, 35, 0).timestamp()
        )));
        // -> 2018-06-01 15:30:00 UTC
        assert!(result.has_own_property(&JsValue::from(
            Utc.ymd(2018, 6, 1).and_hms(15, 30, 0).timestamp()
        )));
        assert!(result.has_own_property(&JsValue::from(
            Utc.ymd(2018, 6, 1).and_hms(15, 35, 0).timestamp()
        )));
    }
}
