#[cfg(test)]
mod tests_time {
    use super::super::Time;

    #[test]
    fn valid_time() {
        let t = Time::new(22, 15, 1, 2, 2003);
        assert_eq!(t.day(), 1);
        assert_eq!(t.month(), 2);
        assert_eq!(t.year(), 2003);
        assert_eq!(t.hrs(), 22);
        assert_eq!(t.mins(), 15);

        // lowest valid time
        Time::new(0, 0, 1, 1, 2000);
        // largest valid time
        Time::new(23, 59, 31, 12, 2099);

        // set bools
        let t2 = Time::new(22, 15, 1, 2, 2003);
        let t3 = Time::new(23, 15, 1, 2, 2003);
        assert_eq!(t, t2);
        assert_ne!(t, t3);
    }

    #[test]
    #[should_panic]
    fn invalid_min() {
        Time::new(0, 60, 20, 3, 2078);
    }

    #[test]
    #[should_panic]
    fn invalid_hrs() {
        Time::new(78, 6, 20, 3, 2078);
    }

    #[test]
    #[should_panic]
    fn invalid_day() {
        Time::new(13, 27, 33, 2, 2000);
    }
    #[test]
    #[should_panic]
    fn invalid_month() {
        Time::new(16, 2, 2, 199, 2040);
    }
    #[test]
    #[should_panic]
    fn invalid_year_over() {
        Time::new(5, 35, 5, 21, 2100);
    }
    #[test]
    #[should_panic]
    fn invalid_year_under() {
        Time::new(4, 49, 5, 21, 1991);
    }

    // test comparisons
    #[test]
    fn test_ordering() {
        use more_asserts::*;
        let t = Time::new(23, 05, 5, 12, 2009);
        assert_lt!(t, Time::new(12, 2, 2, 2, 2022));
        assert_gt!(t, Time::new(7, 1, 13, 8, 2007));
    }

    #[test]
    fn _test_sub() {
        let d1 = Time::new(2, 21, 15, 7, 2015);
        let d2 = Time::new(2, 21, 31, 7, 2015);
        assert_eq!(d2.sub(&d1), 16 * 24 * 60);
        assert_eq!(d1.sub(&d2), 0);
    }
}

#[cfg(test)]
mod test_trip_duration {
    use super::super::{Time, TripDuration};

    #[test]
    fn test_set_1() {
        let mut d = TripDuration::from_minutes(3);
        assert_eq!(
            d,
            TripDuration {
                hours: 3.0 / 60.0,
                days: 0,
                weeks: 0
            }
        );

        d = TripDuration::from_minutes(65);
        assert_eq!(
            d,
            TripDuration {
                hours: 65.0 / 60.0,
                days: 0,
                weeks: 0
            }
        );

        d = TripDuration::from_minutes(90 + 2 * 24 * 60);
        assert_eq!(
            d,
            TripDuration {
                hours: 1.5,
                days: 2,
                weeks: 0
            }
        );

        d = TripDuration::from_minutes(90 + 6 * 24 * 60 + 4 * 7 * 24 * 60);
        assert_eq!(
            d,
            TripDuration {
                hours: 1.5,
                days: 6,
                weeks: 4
            }
        );
    }

    #[test]
    fn test_set_2() {
        let s = Time::new(15, 55, 2, 12, 2005);
        let e = Time::new(22, 29, 14, 12, 2005);
        let d = TripDuration::from_times(&s, &e);
        assert_eq!(
            d,
            TripDuration {
                hours: 6 as f32 + 34.0 / 60.0,
                days: 5,
                weeks: 1
            }
        );
    }
}
