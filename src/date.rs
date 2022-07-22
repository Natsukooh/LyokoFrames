use std::ops::Sub;
use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};

pub const START_DATE: DateTime<Utc> = utc.datetime_from_str(&"Jul 22 00:00:00 2022", "%b %d %H:%M:%S %Y").unwrap();

pub fn day_interval() -> Option<Usize> {
    let now = Utc::now();
    let difference = now.signed_duration_since(START_DATE);

    match difference.num_days() {
        difference if difference >= 0 => Some(difference as usize),
        _ => None
    }
}
