#[cfg(test)]
mod tests {

    use super::super::Cost;
    use super::super::OpenTrip;
    use super::super::Time;
    use assert_float_eq::*;

    #[test]
    fn test_create_open_trip() {
        let now = Time::now();
        let tomorrow = Time::tomorrow();

        let t = OpenTrip::new(now, tomorrow, 30);
        assert_eq!(*t.start_time(), now);
        assert_eq!(*t.end_time(), tomorrow);
        assert_eq!(*t.distance(), 30);
    }

    #[test]
    fn test_setters() {
        let now = Time::now();
        let tomorrow = Time::tomorrow();

        let t = OpenTrip::new(now, tomorrow, 30);
        assert_eq!(
            *t.set_start_time(Time::new(0, 0, 1, 2, 2003)).start_time(),
            Time::new(0, 0, 1, 2, 2003)
        );

        assert_eq!(
            *t.set_start_time(Time::new(0, 0, 1, 2, 2003)).start_time(),
            Time::new(0, 0, 1, 2, 2003)
        );
    }
    #[test]
    #[should_panic]
    fn test_invalid_start() {
        let now = Time::new(19, 42, 1, 8, 2025);
        let tomorrow = Time::new(19, 42, 2, 8, 2025);

        let t = OpenTrip::new(now, tomorrow, 30);
        t.set_end_time(now);
    }
    #[test]
    #[should_panic]
    fn test_invalid_end() {
        let now = Time::now();
        let tomorrow = Time::tomorrow();

        let t = OpenTrip::new(now, tomorrow, 30);
        t.set_start_time(tomorrow);
    }

    #[test]
    #[should_panic]
    fn test_invalid_trip() {
        let now = Time::now();
        let tomorrow = Time::tomorrow();

        OpenTrip::new(tomorrow, now, 30);
    }

    #[test]
    fn test_km_cost() {
        let now = Time::now();
        let tomorrow = Time::tomorrow();

        let mut t = OpenTrip::new(now, tomorrow, 50);
        assert_eq!(t.km_cost(), 0.0);

        t = t.set_distance(175);
        assert_f32_near!(t.km_cost(), 27.0);
    }

    #[test]
    fn test_time_cost() {
        let start = Time::new(16, 32, 3, 8, 2025);
        let end = Time::new(18, 32, 3, 8, 2025);

        // 2 hr trip
        let mut t = OpenTrip::new(start, end, 10);
        assert_f32_near!(t.time_cost(), 27.0);

        // 5 hr trip
        t = OpenTrip::new(start, Time::new(21, 32, 3, 8, 2025), 10);
        assert_f32_near!(t.time_cost(), 55.0);

        // 12 hr trip
        t = OpenTrip::new(start, Time::new(4, 32, 4, 8, 2025), 10);
        assert_f32_near!(t.time_cost(), 55.0);

        // 2-day trip
        t = OpenTrip::new(start, Time::new(16, 32, 5, 8, 2025), 10);
        assert_f32_near!(t.time_cost(), 105.0);

        // 5.5 day trip
        t = OpenTrip::new(start, Time::new(4, 32, 9, 8, 2025), 10);
        assert_f32_near!(t.time_cost(), 305.0);
    }
}
