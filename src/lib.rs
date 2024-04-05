use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

const EPOCH_2000: u64 = 946684800;

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

impl Datetime {
    pub fn new(timestamp: u64) -> Self {
        Self { timestamp }
    }

    pub fn now() -> Self {
        let timestamp = SystemTime::now();
        let duration = timestamp
            .duration_since(UNIX_EPOCH)
            .expect("now should be later than unix epoch");

        Self::new(duration.as_secs())
    }

    pub fn get_epoch_2000_timestamp(&self) -> u64 {
        self.timestamp - EPOCH_2000
    }

    pub fn calculate_gregorian_date_time(&self) -> (u64, u64, u64, u64, u64, u64) {
        let timestamp = self.get_epoch_2000_timestamp();
        let days = timestamp / TS_TO_DAYS;
        let years = days / DAYS_TO_FOURYEARS;
        let days = days % DAYS_TO_FOURYEARS;
        let years = years * 4 + days / DAYS_TO_YEARS;
        let days = days % DAYS_TO_YEARS;
        let months = if years % 4 == 0 { LEAP_MONTHS } else { MONTHS };
        let month = months.into_iter().position(|m| m > days).unwrap();
        let month_start = *months.get(month - 1).unwrap();

        let secs = self.timestamp % TS_TO_DAYS;
        let hour = secs / SECONDS_TO_HOUR;
        let secs = secs % SECONDS_TO_HOUR;
        let min = secs / SECONDS_TO_MINUTES;
        let sec = secs % SECONDS_TO_MINUTES;

        (
            2000 + years,
            month as u64,
            days - month_start + 1,
            hour,
            min,
            sec,
        )
    }

    pub fn format_iso8601(&self) -> String {
        let (year, month, date, hour, min, sec) = self.calculate_gregorian_date_time();
        format!("{year:0>4}-{month:0>2}-{date:0>2}T{hour:0>2}:{min:0>2}:{sec:0>2}+00:00")
    }
}

impl Display for Datetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        assert_eq!(
            (2024, 1, 1, 0, 0, 0),
            Datetime::new(1704067200).calculate_gregorian_date_time(),
        );
        assert_eq!(
            (2024, 4, 1, 0, 0, 0),
            Datetime::new(1711929600).calculate_gregorian_date_time(),
        );
        assert_eq!(
            (2024, 4, 5, 10, 1, 31),
            Datetime::new(1712311291).calculate_gregorian_date_time(),
        );
        // assert_eq!(
        //     (2024, 12, 31, 0, 0, 0),
        //     Datetime::new(1735603200).calculate_gregorian_date_time(),
        // );
    }

    #[test]
    fn it_should_format_dates_according_to_iso8601() {
        assert_eq!(
            "2024-04-05T10:01:31+00:00",
            Datetime::new(1712311291).format_iso8601(),
        );
    }
}
