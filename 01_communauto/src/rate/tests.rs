#[cfg(test)]
mod tests {

    use super::super::super::utils::Time;
    use super::super::Cost;
    use super::super::opentier::OpenTrip;
    use assert_float_eq::*;

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

        // 2 hr OpenTrip
        let mut t = OpenTrip::new(start, end, 10);
        assert_f32_near!(t.time_cost(), 27.0);

        // 5 hr OpenTrip
        t = OpenTrip::new(start, Time::new(21, 32, 3, 8, 2025), 10);
        assert_f32_near!(t.time_cost(), 55.0);

        // 12 hr OpenTrip
        t = OpenTrip::new(start, Time::new(4, 32, 4, 8, 2025), 10);
        assert_f32_near!(t.time_cost(), 55.0);

        // 2-day OpenTrip
        t = OpenTrip::new(start, Time::new(16, 32, 5, 8, 2025), 10);
        assert_f32_near!(t.time_cost(), 105.0);

        // 5.5 day OpenTrip
        t = OpenTrip::new(start, Time::new(4, 32, 9, 8, 2025), 10);
        assert_f32_near!(t.time_cost(), 305.0);
    }
}
