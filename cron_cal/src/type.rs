use bitvec::prelude::*;

pub(crate) const SECONDS_OF_MINUTE: usize = 60;
pub(crate) const MINUTES_OF_HOUR: usize = 60;
pub(crate) const MINUTES_OF_DAY: usize = 24 * MINUTES_OF_HOUR;

pub(crate) type CronCalender = BitArray<[u8; MINUTES_OF_DAY / 8]>;
