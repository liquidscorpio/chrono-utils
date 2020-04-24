//! Utility structs and traits related to chrono's [NaiveDate](https://docs.rs/chrono/0.4.11/chrono/naive/struct.NaiveDate.html)
use crate::oldtime::Duration as OldDuration;
use chrono::{Datelike, NaiveDate};

/// Value at index `i` is the minimum number of days in the month `i+1`
static MONTH_MIN_DAYS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Value at index `i` is the maximum number of days in the month `i+1`
static MONTH_MAX_DAYS: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Common set of methods for transitioning dates into newer ones
pub trait DateTransitions: Sized {
    /// Returns true if leap year
    fn is_leap_year(&self) -> bool;

    /// Returns the last day of the month
    fn last_day_of_month(&self) -> u32;

    /// Returns the date as on the start of the current year
    fn start_of_year(&self) -> Option<Self>;

    /// Returns the date as on the end of the current year
    fn end_of_year(&self) -> Option<Self>;

    /// Returns the date as on the start of the current month
    fn start_of_month(&self) -> Option<Self>;

    /// Returns the date as on the end of the current month
    fn end_of_month(&self) -> Option<Self>;

    /// Returns the date as on the start of the current week
    fn start_of_iso8601_week(&self) -> Option<Self>;

    /// Returns the date as on the end of the current week
    fn end_of_iso8601_week(&self) -> Option<Self>;

    /// Returns the date as on the start of the previous year
    fn start_of_pred_year(&self) -> Option<Self>;

    /// Returns the date as on the end of the previous year
    fn end_of_pred_year(&self) -> Option<Self>;

    /// Returns the date as on the start of the previous month
    fn start_of_pred_month(&self) -> Option<Self>;

    /// Returns the date as on the end of the previous month
    fn end_of_pred_month(&self) -> Option<Self>;

    /// Returns the date as on the start of the previous week
    fn start_of_pred_iso8601_week(&self) -> Option<Self>;

    /// Returns the date as on the end of the previous week
    fn end_of_pred_iso8601_week(&self) -> Option<Self>;

    /// Returns the date as on the start of the succeeding year
    fn start_of_succ_year(&self) -> Option<Self>;

    /// Returns the date as on the end of the succeeding year
    fn end_of_succ_year(&self) -> Option<Self>;

    /// Returns the date as on the start of the succeeding month
    fn start_of_succ_month(&self) -> Option<Self>;

    /// Returns the date as on the end of the succeeding month
    fn end_of_succ_month(&self) -> Option<Self>;

    /// Returns the date as on the start of the succeeding week
    fn start_of_succ_iso8601_week(&self) -> Option<Self>;

    /// Returns the date as on the end of the succeeding week
    fn end_of_succ_iso8601_week(&self) -> Option<Self>;
}

impl DateTransitions for NaiveDate {
    /// Returns true if the date belongs to a year which is leap.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d1 = NaiveDate::from_ymd(1996, 8, 14);
    /// assert_eq!(d1.is_leap_year(), true);
    /// let d2 = NaiveDate::from_ymd(1900, 2, 28);
    /// assert_eq!(d2.is_leap_year(), false);
    /// let d3 = NaiveDate::from_ymd(2000, 2, 29);
    /// assert_eq!(d3.is_leap_year(), true);
    /// let d4 = NaiveDate::from_ymd(1997, 11, 3);
    /// assert_eq!(d4.is_leap_year(), false);
    #[inline]
    fn is_leap_year(&self) -> bool {
        // TODO: Original chrono PR using private APIs
        // self.of().flags().ndays() == 366

        // See: https://github.com/chronotope/chrono/issues/29#issuecomment-84492746
        NaiveDate::from_ymd_opt(self.year(), 2, 29).is_some()
    }

    /// Returns the last day of the month
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d1 = NaiveDate::from_ymd(1996, 2, 23);
    /// assert_eq!(d1.last_day_of_month(), 29);
    /// let d2 = NaiveDate::from_ymd(1993, 2, 1);
    /// assert_eq!(d2.last_day_of_month(), 28);
    /// let d3 = NaiveDate::from_ymd(2000, 1, 1);
    /// assert_eq!(d3.last_day_of_month(), 31);
    #[inline]
    fn last_day_of_month(&self) -> u32 {
        let index = (self.month() - 1) as usize;
        if self.is_leap_year() {
            MONTH_MAX_DAYS[index] as u32
        } else {
            MONTH_MIN_DAYS[index] as u32
        }
    }

    /// Returns the year start date for the current date
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d = NaiveDate::from_ymd(2019, 3, 31);
    /// assert_eq!(d.start_of_year().unwrap(), NaiveDate::from_ymd(2019, 1, 1));
    #[inline]
    fn start_of_year(&self) -> Option<Self> {
        // TODO: Original chrono PR using private APIs
        // self.with_mdf(Mdf::new(1, 1, self.of().flags()))
        NaiveDate::from_ymd_opt(self.year(), 1, 1)
    }

    /// Returns the year end date for the current date
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d = NaiveDate::from_ymd(2019, 3, 31);
    /// assert_eq!(d.end_of_year().unwrap(), NaiveDate::from_ymd(2019, 12, 31));
    #[inline]
    fn end_of_year(&self) -> Option<Self> {
        // TODO: Original chrono PR using private APIs
        // self.with_mdf(Mdf::new(12, 31, self.of().flags()))
        NaiveDate::from_ymd_opt(self.year(), 12, 31)
    }

    /// Returns the start date of the current month
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d = NaiveDate::from_ymd(2019, 9, 13);
    /// assert_eq!(d.start_of_month().unwrap(), NaiveDate::from_ymd(2019, 9, 1));
    #[inline]
    fn start_of_month(&self) -> Option<Self> {
        // TODO: Original chrono PR using private APIs
        // self.with_mdf(Mdf::new(self.month(), 1, self.of().flags()))
        self.with_day(1)
    }

    /// Returns the date as on the end of the current month
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d1 = NaiveDate::from_ymd(1996, 2, 23);
    /// assert_eq!(d1.end_of_month().unwrap(), NaiveDate::from_ymd(1996, 2, 29));
    /// let d2 = NaiveDate::from_ymd(1993, 2, 1);
    /// assert_eq!(d2.end_of_month().unwrap(), NaiveDate::from_ymd(1993, 2, 28));
    /// let d3 = NaiveDate::from_ymd(2000, 1, 1);
    /// assert_eq!(d3.end_of_month().unwrap(), NaiveDate::from_ymd(2000, 1, 31));
    #[inline]
    fn end_of_month(&self) -> Option<Self> {
        self.with_day(self.last_day_of_month())
    }

    /// Returns the start of the week for the current date. Uses the ISO 8601 standard for
    /// calculating the week. See [Wikipedia](https://en.wikipedia.org/wiki/ISO_week_date).
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d1 = NaiveDate::from_ymd(2020, 1, 2);
    /// assert_eq!(d1.start_of_iso8601_week().unwrap(), NaiveDate::from_ymd(2019, 12, 30));
    /// let d2 = NaiveDate::from_ymd(2019, 12, 29);
    /// assert_eq!(d2.start_of_iso8601_week().unwrap(), NaiveDate::from_ymd(2019, 12, 23));
    /// let d3 = NaiveDate::from_ymd(1992, 2, 29);
    /// assert_eq!(d3.start_of_iso8601_week().unwrap(), NaiveDate::from_ymd(1992, 2, 24));
    fn start_of_iso8601_week(&self) -> Option<Self> {
        // TODO: Original chrono PR using private APIs
        // let days = self.of().weekday().num_days_from_monday() as i64;
        let days = self.weekday().num_days_from_monday() as i64;
        self.checked_sub_signed(OldDuration::days(days))
    }

    /// Returns the end of the week for the current date. Uses the ISO 8601 standard for calculating
    /// the week. See [Wikipedia](https://en.wikipedia.org/wiki/ISO_week_date).
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d1 = NaiveDate::from_ymd(2020, 1, 2);
    /// assert_eq!(d1.end_of_iso8601_week().unwrap(), NaiveDate::from_ymd(2020, 1, 5));
    /// let d2 = NaiveDate::from_ymd(2019, 12, 29);
    /// assert_eq!(d2.end_of_iso8601_week().unwrap(), NaiveDate::from_ymd(2019, 12, 29));
    /// let d3 = NaiveDate::from_ymd(1992, 2, 29);
    /// assert_eq!(d3.end_of_iso8601_week().unwrap(), NaiveDate::from_ymd(1992, 3, 1));
    fn end_of_iso8601_week(&self) -> Option<Self> {
        // TODO: Original chrono PR using private APIs
        // let days = 6 - self.of().weekday().num_days_from_monday() as i64;
        let max_days = 6;
        let days = max_days - self.weekday().num_days_from_monday() as i64;
        self.checked_add_signed(OldDuration::days(days))
    }

    /// Returns the start of preceding year relative to the current date
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d = NaiveDate::from_ymd(2019, 3, 31);
    /// assert_eq!(d.start_of_pred_year().unwrap(), NaiveDate::from_ymd(2018, 1, 1));
    #[inline]
    fn start_of_pred_year(&self) -> Option<Self> {
        let prev_year = self.year() - 1;
        // TODO: Original chrono PR using private APIs
        // let flags = YearFlags::from_year(prev_year);
        // NaiveDate::from_mdf(prev_year, Mdf::new(1, 1, flags))

        NaiveDate::from_ymd_opt(prev_year, 1, 1)
    }

    /// Returns the end of preceding year relative to the current date
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d = NaiveDate::from_ymd(2019, 3, 31);
    /// assert_eq!(d.end_of_pred_year().unwrap(), NaiveDate::from_ymd(2018, 12, 31));
    #[inline]
    fn end_of_pred_year(&self) -> Option<Self> {
        let prev_year = self.year() - 1;
        // TODO: Original chrono PR using private APIs
        // let flags = YearFlags::from_year(prev_year);
        // NaiveDate::from_mdf(prev_year, Mdf::new(12, 31, flags))

        NaiveDate::from_ymd_opt(prev_year, 12, 31)
    }

    /// Returns the start of preceding month for the current date.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d1 = NaiveDate::from_ymd(2019, 1, 4);
    /// assert_eq!(d1.start_of_pred_month().unwrap(), NaiveDate::from_ymd(2018, 12, 1));
    /// let d2 = NaiveDate::from_ymd(1999, 11, 17);
    /// assert_eq!(d2.start_of_pred_month().unwrap(), NaiveDate::from_ymd(1999, 10, 1));
    fn start_of_pred_month(&self) -> Option<Self> {
        // TODO: Original chrono PR using private APIs
        // let mut month = self.month() - 1;
        // let mut flags = self.mdf().flags();
        // let mut year = self.year();
        // if month == 0 {
        //     month = 12;
        //     year -= 1;
        //     flags = YearFlags::from_year(year);
        // };
        // NaiveDate::from_mdf(year, Mdf::new(month, 1, flags))

        let mut month = self.month() - 1;
        let mut year = self.year();
        if month == 0 {
            month = 12;
            year -= 1;
        };
        NaiveDate::from_ymd_opt(year, month, 1)
    }

    /// Returns the end of preceding month for the current date.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d1 = NaiveDate::from_ymd(2019, 1, 4);
    /// assert_eq!(d1.end_of_pred_month().unwrap(), NaiveDate::from_ymd(2018, 12, 31));
    /// let d2 = NaiveDate::from_ymd(1999, 10, 17);
    /// assert_eq!(d2.end_of_pred_month().unwrap(), NaiveDate::from_ymd(1999, 9, 30));
    /// let d3 = NaiveDate::from_ymd(1996, 3, 1);
    /// assert_eq!(d3.end_of_pred_month().unwrap(), NaiveDate::from_ymd(1996, 2, 29));
    fn end_of_pred_month(&self) -> Option<Self> {
        match self.start_of_pred_month() {
            Some(pred_start) => pred_start.with_day(pred_start.last_day_of_month()),
            None => None,
        }
    }

    /// Returns the start of preceding week for the current date. Uses the ISO 8601 standard for
    /// calculating the week. See [Wikipedia](https://en.wikipedia.org/wiki/ISO_week_date).
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d1 = NaiveDate::from_ymd(2019, 1, 4);
    /// assert_eq!(d1.start_of_pred_iso8601_week().unwrap(), NaiveDate::from_ymd(2018, 12, 24));
    /// let d2 = NaiveDate::from_ymd(1999, 10, 17);
    /// assert_eq!(d2.start_of_pred_iso8601_week().unwrap(), NaiveDate::from_ymd(1999, 10, 4));
    /// let d3 = NaiveDate::from_ymd(1996, 3, 1);
    /// assert_eq!(d3.start_of_pred_iso8601_week().unwrap(), NaiveDate::from_ymd(1996, 2, 19));
    fn start_of_pred_iso8601_week(&self) -> Option<Self> {
        match self.start_of_iso8601_week() {
            Some(week_start) => Some(week_start - OldDuration::days(7)),
            None => None,
        }
    }

    /// Returns the end of preceding week for the current date. Uses the ISO 8601 standard for
    /// calculating the week. See [Wikipedia](https://en.wikipedia.org/wiki/ISO_week_date).
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d1 = NaiveDate::from_ymd(2019, 1, 4);
    /// assert_eq!(d1.end_of_pred_iso8601_week().unwrap(), NaiveDate::from_ymd(2018, 12, 30));
    /// let d2 = NaiveDate::from_ymd(1999, 10, 17);
    /// assert_eq!(d2.end_of_pred_iso8601_week().unwrap(), NaiveDate::from_ymd(1999, 10, 10));
    /// let d3 = NaiveDate::from_ymd(1996, 3, 1);
    /// assert_eq!(d3.end_of_pred_iso8601_week().unwrap(), NaiveDate::from_ymd(1996, 2, 25));
    fn end_of_pred_iso8601_week(&self) -> Option<Self> {
        match self.start_of_iso8601_week() {
            Some(week_start) => Some(week_start - OldDuration::days(1)),
            None => None,
        }
    }

    /// Returns the start of succeeding year relative to the current date
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d = NaiveDate::from_ymd(2019, 3, 31);
    /// assert_eq!(d.start_of_succ_year().unwrap(), NaiveDate::from_ymd(2020, 1, 1));
    #[inline]
    fn start_of_succ_year(&self) -> Option<Self> {
        let nxt_year = self.year() + 1;
        // TODO: Original chrono PR using private APIs
        // let flags = YearFlags::from_year(nxt_year);
        // NaiveDate::from_mdf(nxt_year, Mdf::new(1, 1, flags))
        NaiveDate::from_ymd_opt(nxt_year, 1, 1)
    }

    /// Returns the end of succeeding year relative to the current date
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d = NaiveDate::from_ymd(2019, 3, 31);
    /// assert_eq!(d.end_of_succ_year().unwrap(), NaiveDate::from_ymd(2020, 12, 31));
    #[inline]
    fn end_of_succ_year(&self) -> Option<Self> {
        let nxt_year = self.year() + 1;
        // TODO: Original chrono PR using private APIs
        // let flags = YearFlags::from_year(nxt_year);
        // NaiveDate::from_mdf(prev_year, Mdf::new(12, 31, flags))
        NaiveDate::from_ymd_opt(nxt_year, 12, 31)
    }

    /// Returns the start of succeeding month for the current date.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d1 = NaiveDate::from_ymd(2019, 12, 12);
    /// assert_eq!(d1.start_of_succ_month().unwrap(), NaiveDate::from_ymd(2020, 1, 1));
    /// let d2 = NaiveDate::from_ymd(1999, 2, 28);
    /// assert_eq!(d2.start_of_succ_month().unwrap(), NaiveDate::from_ymd(1999, 3, 1));
    fn start_of_succ_month(&self) -> Option<Self> {
        // TODO: Original chrono PR using private APIs
        // let mut month = self.month() + 1;
        // let mut flags = self.mdf().flags();
        // let mut year = self.year();
        // if month == 13 {
        //     month = 1;
        //     year += 1;
        //     flags = YearFlags::from_year(year);
        // };
        // NaiveDate::from_mdf(year, Mdf::new(month, 1, flags))

        let mut month = self.month() + 1;
        let mut year = self.year();
        if month == 13 {
            month = 1;
            year += 1;
        };
        NaiveDate::from_ymd_opt(year, month, 1)
    }

    /// Returns the end of succeeding month for the current date.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d1 = NaiveDate::from_ymd(2019, 1, 4);
    /// assert_eq!(d1.end_of_pred_month().unwrap(), NaiveDate::from_ymd(2018, 12, 31));
    /// let d2 = NaiveDate::from_ymd(1999, 10, 17);
    /// assert_eq!(d2.end_of_pred_month().unwrap(), NaiveDate::from_ymd(1999, 9, 30));
    /// let d3 = NaiveDate::from_ymd(1996, 3, 1);
    /// assert_eq!(d3.end_of_pred_month().unwrap(), NaiveDate::from_ymd(1996, 2, 29));
    fn end_of_succ_month(&self) -> Option<Self> {
        match self.start_of_succ_month() {
            Some(succ_start) => succ_start.with_day(succ_start.last_day_of_month()),
            None => None,
        }
    }

    /// Returns the start of succeeding week for the current date. Uses the ISO 8601 standard for
    /// calculating the week. See [Wikipedia](https://en.wikipedia.org/wiki/ISO_week_date).
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d1 = NaiveDate::from_ymd(2020, 1, 4);
    /// assert_eq!(d1.start_of_succ_iso8601_week().unwrap(), NaiveDate::from_ymd(2020, 1, 6));
    /// let d2 = NaiveDate::from_ymd(2017, 12, 28);
    /// assert_eq!(d2.start_of_succ_iso8601_week().unwrap(), NaiveDate::from_ymd(2018, 1, 1));
    /// let d3 = NaiveDate::from_ymd(1996, 2, 26);
    /// assert_eq!(d3.start_of_succ_iso8601_week().unwrap(), NaiveDate::from_ymd(1996, 3, 4));
    fn start_of_succ_iso8601_week(&self) -> Option<Self> {
        match self.start_of_iso8601_week() {
            Some(week_start) => Some(week_start + OldDuration::days(7)),
            None => None,
        }
    }

    /// Returns the end of succeeding week for the current date. Uses the ISO 8601 standard for
    /// calculating the week. See [Wikipedia](https://en.wikipedia.org/wiki/ISO_week_date).
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    /// use chrono_utils::naive::DateTransitions;
    ///
    /// let d1 = NaiveDate::from_ymd(2019, 1, 4);
    /// assert_eq!(d1.end_of_succ_iso8601_week().unwrap(), NaiveDate::from_ymd(2019, 1, 13));
    /// let d2 = NaiveDate::from_ymd(2004, 2, 20);
    /// assert_eq!(d2.end_of_succ_iso8601_week().unwrap(), NaiveDate::from_ymd(2004, 2, 29));
    /// let d3 = NaiveDate::from_ymd(2005, 12, 20);
    /// assert_eq!(d3.end_of_succ_iso8601_week().unwrap(), NaiveDate::from_ymd(2006, 1, 1));
    fn end_of_succ_iso8601_week(&self) -> Option<Self> {
        match self.start_of_succ_iso8601_week() {
            Some(week_start) => Some(week_start + OldDuration::days(6)),
            None => None,
        }
    }
}
