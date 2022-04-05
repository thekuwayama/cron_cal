#![allow(clippy::unused_unit)]

use chrono::offset::Utc;
use cron_cal::format::format_unix_timestamp;
use cron_cal::parse::parse;
use js_sys::{Array, BigInt, BigInt64Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_cron_cal(input: &str) -> BigInt64Array {
    // TODO: Add args; days, date
    let date = Utc::today().and_hms(0, 0, 0);
    let days = 1;
    let mut input = input.as_bytes();

    let cal = parse(&mut input, date, days).unwrap(); // FIXME: error handling
    BigInt64Array::new(&JsValue::from(
        format_unix_timestamp(&cal, date)
            .iter()
            .flat_map(|p| vec![p.0, p.1])
            .map(BigInt::from)
            .collect::<Array>(),
    ))
}
