#[cfg(test)]
mod tests {

    use super::super::Cost;
    use super::super::OpenTrip;
    use super::super::Time;
    use more_asserts::*;

    #[test]
    fn test_create_open_trip() {
        let now = Time::new(19, 42, 1, 8, 2025);
        let tomorrow = Time::new(19, 42, 2, 8, 2025);

        let t = OpenTrip::new(now, tomorrow, 30);
        assert_eq!(*t.start_time(), now);
        assert_eq!(*t.end_time(), tomorrow);
        assert_eq!(*t.distance(), 30);
    }

    #[test]
    #[should_panic]
    fn test_invalid_trip() {
        let now = Time::new(20, 2, 1, 8, 2025);
        let tomorrow = Time::new(20, 2, 2, 8, 2025);

        OpenTrip::new(tomorrow, now, 30);
    }

    #[test]
    fn test_km_cost() {
        let now = Time::new(20, 3, 1, 8, 2025);
        let tomorrow = Time::new(20, 3, 2, 8, 2025);

        let mut t = OpenTrip::new(now, tomorrow, 50);
        assert_eq!(t.km_cost(), 0.0);

        t = t.set_distance(175);
        assert_le!((t.km_cost() - 27.0).abs(), f32::EPSILON);
    }
}
