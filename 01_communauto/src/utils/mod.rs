mod tests;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Time {
    day: u32,
    month: u32,
    year: u32,

    hrs: u32,
    mins: u32,
}
fn is_leap_year(year: &u32) -> bool {
    (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}
impl Time {
    pub fn new(hrs: u32, mins: u32, day: u32, month: u32, year: u32) -> Self {
        // A valid time must be after Jan 1, 2000 and before Jan 1, 2100

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
            panic!("Invalid minutes, got {:?}", mins);
        }

        Self {
            day: day,
            month: month,
            year: year,
            hrs: hrs,
            mins: mins,
        }
    }
    pub fn now() -> Self {
        use chrono::{DateTime, Datelike, Local, Timelike};
        let now: DateTime<Local> = Local::now();
        Self {
            hrs: now.hour(),
            mins: now.minute(),
            day: now.day(),
            month: now.month(),
            year: now.year() as u32,
        }
    }
    pub fn tomorrow() -> Self {
        use chrono::{DateTime, Datelike, Days, Local, Timelike};
        let tomorrow: DateTime<Local> = Local::now() + Days::new(1);
        Self {
            hrs: tomorrow.hour(),
            mins: tomorrow.minute(),
            day: tomorrow.day(),
            month: tomorrow.month(),
            year: tomorrow.year() as u32,
        }
    }
    pub fn day(&self) -> u32 {
        self.day
    }
    pub fn month(&self) -> u32 {
        self.month
    }
    pub fn year(&self) -> u32 {
        self.year
    }
    pub fn hrs(&self) -> u32 {
        self.hrs
    }
    pub fn mins(&self) -> u32 {
        self.mins
    }

    pub fn sub(&self, other: &Self) -> u32 {
        // expects other <= self
        // if other > self, then return 0

        self.as_mins().saturating_sub(other.as_mins())
    }

    pub fn as_mins(&self) -> u32 {
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

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.as_mins().cmp(&other.as_mins()))
    }
}

#[derive(PartialEq, Debug)]
pub struct TripDuration {
    hours: f32,
    days: u32,
    weeks: u32,
}
impl TripDuration {
    pub fn from_minutes(minutes: u32) -> Self {
        Self {
            weeks: minutes / (7 * 24 * 60),
            days: (minutes % (7 * 24 * 60)) / (24 * 60),
            hours: ((minutes % (7 * 24 * 60)) % (24 * 60)) as f32 / 60.0,
        }
    }
    pub fn from_times(start_time: &Time, end_time: &Time) -> Self {
        if start_time >= end_time {
            panic!(
                "start time {:?} cannot be after end time {:?}",
                start_time, end_time
            )
        }
        Self::from_minutes(end_time.as_mins() - start_time.as_mins())
    }
    pub fn hours(&self) -> &f32 {
        &self.hours
    }
    pub fn days(&self) -> &u32 {
        &self.days
    }
    pub fn weeks(&self) -> &u32 {
        &self.weeks
    }
}
