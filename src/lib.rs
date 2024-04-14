#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{
    fmt::{self, Display, Formatter},
    format,
    string::String,
};

#[cfg(feature = "std")]
use std::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::time::{SystemTime, UNIX_EPOCH};

/// 2001-01-01 as epoch timestamp
const EPOCH_2001: u64 = 978307200;

const TS_TO_DAYS: u64 = 24 * 60 * 60;
const SECONDS_TO_HOUR: u64 = 60 * 60;
const SECONDS_TO_MINUTES: u64 = 60;
const DAYS_TO_FOURYEARS: u64 = 365 * 4 + 1;
const DAYS_TO_YEARS: u64 = 365;

const MONTHS: [u64; 13] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];
const LEAP_MONTHS: [u64; 13] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366];

#[derive(Debug)]
pub struct Datetime {
    timestamp: u64,
}

fn zeropad(mut number: u64, n: usize) -> String {
    let mut s = String::with_capacity(n);
    for i in 0..n {
        let p = 10u64.pow((n - i - 1) as u32);
        let rem = number / p;
        dbg!(number, p, rem);
        s.push(char::from_digit(rem as u32, 10).unwrap());
        number -= p * rem;
    }
    s
}

impl Datetime {
    pub fn new(timestamp: u64) -> Self {
        Self { timestamp }
    }

    #[cfg(feature = "std")]
    pub fn now() -> Self {
        let timestamp = SystemTime::now();
        let duration = timestamp
            .duration_since(UNIX_EPOCH)
            .expect("now should be later than unix epoch");

        Self::new(duration.as_secs())
    }

    pub fn date(&self) -> (u64, u64, u64) {
        let timestamp = self.timestamp - EPOCH_2001;
        let days = timestamp / TS_TO_DAYS;
        let years = days / DAYS_TO_FOURYEARS;
        let days = days % DAYS_TO_FOURYEARS;

        // The last day for the leap year is the 366th, this is better to be handled separately
        if days == DAYS_TO_FOURYEARS - 1 {
            let years = 2004 + years * 4;
            return (years, 12, 31);
        }

        let years = 2001 + years * 4 + days / DAYS_TO_YEARS;
        let days = days % DAYS_TO_YEARS;
        let months = if years % 4 == 0 { LEAP_MONTHS } else { MONTHS };
        let month = months.into_iter().position(|m| m > days).unwrap();
        let month_start = *months.get(month - 1).unwrap();

        (years, month as u64, days - month_start + 1)
    }

    pub fn time(&self) -> (u64, u64, u64) {
        let secs = self.timestamp % TS_TO_DAYS;
        let hour = secs / SECONDS_TO_HOUR;
        let secs = secs % SECONDS_TO_HOUR;
        let min = secs / SECONDS_TO_MINUTES;
        let sec = secs % SECONDS_TO_MINUTES;

        (hour, min, sec)
    }

    pub fn format_iso8601(&self) -> String {
        let (year, month, day) = self.date();
        let (hour, min, sec) = self.time();
        format!("{year:0>4}-{month:0>2}-{day:0>2}T{hour:0>2}:{min:0>2}:{sec:0>2}Z")
    }
}

impl Display for Datetime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format_iso8601())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_return_a_datetime() {
        let ts = 1712224891;
        let dt = Datetime::new(ts);
        assert_eq!(ts, dt.timestamp);
    }

    #[test]
    fn it_should_calculate_gregorian_dates() {
        assert_eq!((2023, 1, 1), Datetime::new(1672531200).date());
        assert_eq!((2023, 1, 31), Datetime::new(1675123200).date());
        assert_eq!((2023, 2, 28), Datetime::new(1677542400).date());
        assert_eq!((2023, 3, 1), Datetime::new(1677628800).date());
        assert_eq!((2023, 12, 31), Datetime::new(1703980800).date());
        assert_eq!((2024, 1, 1), Datetime::new(1704067200).date());
        assert_eq!((2024, 1, 31), Datetime::new(1706659200).date());
        assert_eq!((2024, 2, 28), Datetime::new(1709078400).date());
        assert_eq!((2024, 2, 29), Datetime::new(1709164800).date());
        assert_eq!((2024, 3, 1), Datetime::new(1709251200).date());
        assert_eq!((2024, 4, 1), Datetime::new(1711929600).date());
        assert_eq!((2024, 4, 5), Datetime::new(1712311291).date());
        assert_eq!((2024, 12, 30), Datetime::new(1735516800).date());
        assert_eq!((2024, 12, 31), Datetime::new(1735603200).date());
    }

    #[test]
    fn it_should_calculate_times() {
        assert_eq!((0, 0, 0), Datetime::new(1672531200).time());
        assert_eq!((10, 1, 31), Datetime::new(1712311291).time());
        assert_eq!((11, 59, 59), Datetime::new(1712318399).time());
        assert_eq!((23, 59, 59), Datetime::new(1712361599).time());
    }

    #[test]
    fn it_should_format_dates_according_to_iso8601() {
        assert_eq!(
            "2024-04-05T10:01:31Z",
            Datetime::new(1712311291).format_iso8601(),
        );
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    use time::format_description::well_known::{
        iso8601::{Config, TimePrecision},
        Iso8601,
    };

    const MAX_2099: u64 = 4102444799;

    proptest! {
        #[test]
        fn it_should_be_the_same_as_chrono(ts in EPOCH_2001..MAX_2099) {
            let chrono_date = chrono::DateTime::from_timestamp(ts as i64, 0)
                .unwrap()
                .format("%Y-%m-%dT%H:%M:%SZ")
                .to_string();
            let date = Datetime::new(ts).to_string();
            assert_eq!(chrono_date, date);
        }
    }

    proptest! {
        #[test]
        fn it_should_be_the_same_as_time(
            ts in EPOCH_2001..MAX_2099
        ) {
            const ISO: u128 = Config::DEFAULT.set_time_precision(TimePrecision::Second { decimal_digits: None }).encode();
            let chrono_date = time::OffsetDateTime::from_unix_timestamp(ts as i64)
                .unwrap()
                .format(&Iso8601::<ISO>)
                .unwrap();
            let date = Datetime::new(ts).to_string();
            assert_eq!(chrono_date, date);
        }
    }
}
