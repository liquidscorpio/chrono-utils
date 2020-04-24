//! Provides utility functions to manipulate [chrono](https://github.com/chronotope/chrono/) dates.
//! Only [NaiveDate](https://docs.rs/chrono/0.4.11/chrono/naive/struct.NaiveDate.html) is
//! supported as of now. Support for naive and timezone aware DateTime coming soon.
//!
//! The crate provides the following:
//!
//! **Transition APIs**
//! Transition a chrono struct into a future or previous date using standardised methods
//! like `start_of_pred_iso8601_week()` which provides the date on which the previous week
//! starts. Such functions are provided for week, month and year.

extern crate chrono;
extern crate time as oldtime;

pub mod naive;


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::naive::DateTransitions;

    #[test]
    fn test_api_interface() {
        let d1 = NaiveDate::from_ymd(1996, 2, 23);
        // Month
        assert_eq!(d1.end_of_month().unwrap(), NaiveDate::from_ymd(1996, 2, 29));
        assert_eq!(d1.start_of_month().unwrap(), NaiveDate::from_ymd(1996, 2, 1));
        assert_eq!(d1.end_of_pred_month().unwrap(), NaiveDate::from_ymd(1996, 1, 31));
        assert_eq!(d1.start_of_pred_month().unwrap(), NaiveDate::from_ymd(1996, 1, 1));
        assert_eq!(d1.end_of_succ_month().unwrap(), NaiveDate::from_ymd(1996, 3, 31));
        assert_eq!(d1.start_of_succ_month().unwrap(), NaiveDate::from_ymd(1996, 3, 1));

        // Year
        assert_eq!(d1.end_of_year().unwrap(), NaiveDate::from_ymd(1996, 12, 31));
        assert_eq!(d1.start_of_year().unwrap(), NaiveDate::from_ymd(1996, 1, 1));
        assert_eq!(d1.end_of_pred_year().unwrap(), NaiveDate::from_ymd(1995, 12, 31));
        assert_eq!(d1.start_of_pred_year().unwrap(), NaiveDate::from_ymd(1995, 1, 1));
        assert_eq!(d1.end_of_succ_year().unwrap(), NaiveDate::from_ymd(1997, 12, 31));
        assert_eq!(d1.start_of_succ_year().unwrap(), NaiveDate::from_ymd(1997, 1, 1));

        // ISO 8601 Week
        assert_eq!(d1.end_of_iso8601_week().unwrap(), NaiveDate::from_ymd(1996, 2, 25));
        assert_eq!(d1.start_of_iso8601_week().unwrap(), NaiveDate::from_ymd(1996, 2, 19));
        assert_eq!(d1.end_of_pred_iso8601_week().unwrap(), NaiveDate::from_ymd(1996, 2, 18));
        assert_eq!(d1.start_of_pred_iso8601_week().unwrap(), NaiveDate::from_ymd(1996, 2, 12));
        assert_eq!(d1.end_of_succ_iso8601_week().unwrap(), NaiveDate::from_ymd(1996, 3, 3));
        assert_eq!(d1.start_of_succ_iso8601_week().unwrap(), NaiveDate::from_ymd(1996, 2, 26));

        // Leap year
        assert_eq!(d1.is_leap_year(), true);
        let d2 = NaiveDate::from_ymd(1900, 7, 4);
        assert_eq!(d2.is_leap_year(), false);
    }
}
