#[cfg(test)]
mod tests {
    use super::super::Date;

    #[test]
    fn valid_date() {
        let date = Date::new(22, 15, 1, 2, 2003);
        assert_eq!(date.day(), 1);
        assert_eq!(date.month(), 2);
        assert_eq!(date.year(), 2003);
        assert_eq!(date.hrs(), 22);
        assert_eq!(date.mins(), 15);

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
