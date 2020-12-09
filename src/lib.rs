#![crate_name = "date_iter"]

extern crate chrono;
use chrono::Duration;

pub type Date = chrono::naive::NaiveDate;

/// Iterator for dates
#[derive(Debug)]
pub struct DateIter {
    cur: Date,
    stop: Date,
}

impl Iterator for DateIter {
    /// item is naive date
    type Item = Date;

    /// return next date
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur <= self.stop {
            let cur = Some(self.cur);
            self.cur += Duration::days(1);
            cur
        } else {
            None
        }
    }
}

pub fn date_iter(start: Date, stop: Date) -> DateIter {
    DateIter {
        cur: start,
        stop: stop,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let di = date_iter(Date::from_ymd(2020, 12, 4), Date::from_ymd(2020, 12, 8));
        let actual: Vec<Date> = di.collect();
        let expected: Vec<Date> = (4..9).map(|d| Date::from_ymd(2020, 12, d)).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn step() {
        let di = date_iter(Date::from_ymd(2020, 12, 1), Date::from_ymd(2020, 12, 8));
        let actual: Vec<Date> = di.step_by(2).collect();
        let expected: Vec<Date> = (1..8)
            .step_by(2)
            .into_iter()
            .map(|d| Date::from_ymd(2020, 12, d))
            .collect();
        assert_eq!(actual, expected);
    }
}
