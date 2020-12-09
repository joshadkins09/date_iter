#![crate_name = "date_iter"]

use chrono::naive::NaiveDate;
use chrono::Duration;

pub type DatePair = (NaiveDate, NaiveDate);

/// Iterator for dates
#[derive(Debug)]
pub struct DateIter {
    cur: NaiveDate,
    stop: NaiveDate,
}

impl DateIter {
    fn new(start: NaiveDate, stop: NaiveDate) -> Self {
        Self {
            cur: start,
            stop: stop,
        }
    }
}

impl Iterator for DateIter {
    /// item is naive date
    type Item = NaiveDate;

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

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let di = DateIter::new(
            NaiveDate::from_ymd(2020, 12, 4),
            NaiveDate::from_ymd(2020, 12, 8),
        );
        let actual: Vec<NaiveDate> = di.collect();
        let expected: Vec<NaiveDate> = (4..9).map(|d| NaiveDate::from_ymd(2020, 12, d)).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn interval() {
        let di = DateIter::new(
            NaiveDate::from_ymd(2020, 12, 1),
            NaiveDate::from_ymd(2020, 12, 8),
        );
        let actual: Vec<NaiveDate> = di.step_by(2).collect();
        let expected: Vec<NaiveDate> = (1..8)
            .step_by(2)
            .into_iter()
            .map(|d| NaiveDate::from_ymd(2020, 12, d))
            .collect();
        assert_eq!(actual, expected);
    }
}
