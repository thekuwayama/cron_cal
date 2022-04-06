#![allow(clippy::unused_unit)]

use chrono::naive::NaiveDateTime;
use chrono::{DateTime, Utc};
use cron_cal::format::format_unix_timestamp;
use cron_cal::parse::parse;
use js_sys::{Array, BigInt, BigInt64Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_cron_cal(input: &str, date: i64, days: i32) -> Result<BigInt64Array, JsValue> {
    let date = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(date, 0), Utc);
    let mut input = input.as_bytes();

    match parse(&mut input, date, days as usize) {
        Ok(cal) => Ok(BigInt64Array::new(&JsValue::from(
            format_unix_timestamp(&cal, date)
                .iter()
                .flat_map(|p| vec![p.0, p.1])
                .map(BigInt::from)
                .collect::<Array>(),
        ))),
        _ => Err(JsValue::from("cron format error")),
    }
}
