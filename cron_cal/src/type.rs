use std::convert::From;

use bitvec::prelude::*;

pub(crate) const SECONDS_OF_MINUTE: usize = 60;
pub(crate) const MINUTES_OF_HOUR: usize = 60;
pub(crate) const MINUTES_OF_DAY: usize = 24 * MINUTES_OF_HOUR;

pub(crate) type CronCalender = BitArray<[u8; MINUTES_OF_DAY / 8]>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CronSchedule {
    pub start: i64,
    pub end: i64,
}

impl From<(i64, i64)> for CronSchedule {
    fn from(pair: (i64, i64)) -> Self {
        CronSchedule {
            start: pair.0,
            end: pair.1,
        }
    }
}
