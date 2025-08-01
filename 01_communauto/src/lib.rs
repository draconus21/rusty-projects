#[derive(PartialEq, Debug)]
pub struct Date {
    day: u32,
    month: u32,
    year: u32,

    hrs: u32,
    mins: u32,
}
fn is_leap_year(year: &u32) -> bool {
    (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}
impl Date {
    fn new(hrs: u32, mins: u32, day: u32, month: u32, year: u32) -> Self {
        // A valid date must be after Jan 1, 2000 and before Jan 1, 2100

        if year < 2000 || year >= 2100 {
            panic!("Year must be between 2000 and 2099, got {:?}", year);
        }
        if month < 1 || month > 12 {
            panic!("Invalid month, got {:?}", month);
        }

        if day > 31 {
            panic!("Invalid day, got {:?}", day);
        } else if [4, 6, 9, 11].contains(&month) && day > 30 {
            panic!("Invalid day for month {:?}, got {:?}", month, day);
        } else if month == 2 {
            if is_leap_year(&year) && day > 29 {
                panic!("Invalid day for leap month {:?}, got {:?}", month, day);
            } else if day > 28 {
                panic!("Invalid day for month {:?}, got {:?}", month, day);
            }
        }

        if hrs > 23 {
            panic!("Invalid hours, got {:?}", hrs);
        }
        if mins > 59 {
            panic!("Invalid minutes, got {:?}", mins)
        }

        Self {
            day: day,
            month: month,
            year: year,
            hrs: hrs,
            mins: mins,
        }
    }

    fn sub(&self, other: &Self) -> u32 {
        // expects other <= self
        // if other > self, then return 0

        self.as_mins().saturating_sub(other.as_mins())
    }

    fn as_mins(&self) -> u32 {
        // Assmue that time starts on Jan 1, 2000

        let mut n_days: u32 = 0;

        // number of years
        let diff = self.year - 2000;
        n_days += diff * 365;
        // find number of leap years in between
        let leap_years = diff / 4 - diff / 100 + diff / 400;
        n_days += leap_years;

        for m in 1..self.month {
            n_days += match m {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 32,
                4 | 6 | 9 | 11 => 31,
                2 => {
                    if is_leap_year(&self.year) {
                        29
                    } else {
                        28
                    }
                }
                _ => panic!("invalid month {:?}", m),
            }
        }

        n_days += self.day;

        let mins = self.mins + 60 * self.hrs;

        n_days * 24 * 60 + mins
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.as_mins().cmp(&other.as_mins()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_date() {
        let date = Date::new(22, 15, 1, 2, 2003);
        assert_eq!(date.day, 1);
        assert_eq!(date.month, 2);
        assert_eq!(date.year, 2003);
        assert_eq!(date.hrs, 22);
        assert_eq!(date.mins, 15);

        // lowest valid date
        Date::new(0, 0, 1, 1, 2000);
        // largest valid date
        Date::new(23, 59, 31, 12, 2099);
    }

    #[test]
    #[should_panic]
    fn invalid_min() {
        Date::new(0, 60, 20, 3, 2078);
    }

    #[test]
    #[should_panic]
    fn invalid_hrs() {
        Date::new(78, 6, 20, 3, 2078);
    }

    #[test]
    #[should_panic]
    fn invalid_day() {
        Date::new(13, 27, 33, 2, 2000);
    }
    #[test]
    #[should_panic]
    fn invalid_month() {
        Date::new(16, 2, 2, 199, 2040);
    }
    #[test]
    #[should_panic]
    fn invalid_year_over() {
        Date::new(5, 35, 5, 21, 2100);
    }
    #[test]
    #[should_panic]
    fn invalid_year_under() {
        Date::new(4, 49, 5, 21, 1991);
    }

    // test comparisons
    #[test]
    fn test_ordering() {
        use more_asserts::*;
        let date = Date::new(23, 05, 5, 12, 2009);
        assert_lt!(date, Date::new(12, 2, 2, 2, 2022));
        assert_gt!(date, Date::new(7, 1, 13, 8, 2007));
    }

    #[test]
    fn _test_sub() {
        let d1 = Date::new(2, 21, 15, 7, 2015);
        let d2 = Date::new(2, 21, 31, 7, 2015);
        assert_eq!(d2.sub(&d1), 16 * 24 * 60);
        assert_eq!(d1.sub(&d2), 0);
    }
}
