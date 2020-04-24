## Chrono Utilities

**Documentation**: https://docs.rs/chrono-utilities/  
**Current Version**: 0.0.0-alpha1

----
<br>
Provides utility functions to manipulate [chrono](https://github.com/chronotope/chrono/) dates.
Only [NaiveDate](https://docs.rs/chrono/0.4.11/chrono/naive/struct.NaiveDate.html) is
supported as of now. Support for naive and timezone aware DateTime coming soon.

The crate provides the following:    
  
**Transition APIs**    

Transition a chrono struct into a future or previous date using standardised methods
like `start_of_pred_iso8601_week()` which provides the date on which the previous week
starts. Such functions are provided for week, month and year. View the [docs](https://docs.rs/chrono-utilities/0.0.0-alpha1/chrono_utilities/naive/trait.DateTransitions.html) for full
API and more examples.


<br>
 
### Feature Matrix
| Feature | Status |
| -------------- | --------- |
| Transition APIs for NaiveDate | ✓ |
| Transition APIs for Aware Dates | 𐄂 |
| Transition APIs for NaiveDateTime | 𐄂 |
| Transition APIs for Aware DateTime | 𐄂 |


### Code Sample
```rust
use chrono::NaiveDate;
use chrono_utils::naive::DateTransitions;

let d = NaiveDate::from_ymd(2019, 3, 31);
assert_eq!(d.start_of_succ_year().unwrap(), NaiveDate::from_ymd(2020, 1, 1));

let d1 = NaiveDate::from_ymd(1996, 2, 23);
assert_eq!(d1.last_day_of_month(), 29);

let d2 = NaiveDate::from_ymd(1999, 10, 17);
assert_eq!(d2.start_of_pred_iso8601_week().unwrap(), NaiveDate::from_ymd(1999, 10, 4));
```

### Licence
Dual licensed under Apache 2.0 and MIT. (Same as Rust and chrono.)
