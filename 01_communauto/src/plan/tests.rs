#[cfg(test)]

mod test {

    use crate::utils::{Time, Trip};

    use super::super::{OPEN_PLUS_TRIP, OPEN_TRIP};
    use assert_float_eq::*;

    #[test]
    fn test_km_cost() {
        assert_eq!(OPEN_TRIP.km_cost(50), 0.0);
        assert_f32_near!(OPEN_PLUS_TRIP.km_cost(50), 12.5);

        assert_f32_near!(OPEN_TRIP.km_cost(175), 27.0);
        assert_f32_near!(OPEN_PLUS_TRIP.km_cost(175), 40.0);
    }

    #[test]
    fn test_time_cost() {
        let start = Time::new(16, 32, 3, 8, 2025);
        let end = Time::new(18, 32, 3, 8, 2025);

        // 2 hr Trip
        let mut t = Trip::new(start, end, 10);
        assert_f32_near!(OPEN_TRIP.time_cost(t.trip_time()), 27.0);
        assert_f32_near!(OPEN_PLUS_TRIP.time_cost(t.trip_time()), 13.7);

        // 5 hr Trip
        t = Trip::new(start, Time::new(21, 32, 3, 8, 2025), 10);
        assert_f32_near!(OPEN_TRIP.time_cost(t.trip_time()), 55.0);
        assert_f32_near!(OPEN_PLUS_TRIP.time_cost(t.trip_time()), 34.25);

        // 12 hr Trip
        t = Trip::new(start, Time::new(4, 32, 4, 8, 2025), 10);
        assert_f32_near!(OPEN_TRIP.time_cost(t.trip_time()), 55.0);
        assert_f32_near!(OPEN_PLUS_TRIP.time_cost(t.trip_time()), 50.0);

        // 2-day Trip
        t = Trip::new(start, Time::new(16, 32, 5, 8, 2025), 10);
        assert_f32_near!(OPEN_TRIP.time_cost(t.trip_time()), 105.0);
        assert_f32_near!(OPEN_PLUS_TRIP.time_cost(t.trip_time()), 85.0);

        // 5.5 day Trip
        t = Trip::new(start, Time::new(4, 32, 9, 8, 2025), 10);
        assert_f32_near!(OPEN_TRIP.time_cost(t.trip_time()), 305.0);
        assert_f32_near!(OPEN_PLUS_TRIP.time_cost(t.trip_time()), 225.0);
    }
    #[test]
    fn test_total_cost() {
        let start = Time::new(16, 32, 3, 8, 2025);
        let end = Time::new(18, 32, 3, 8, 2025);

        // 2 hr OpenTrip
        let t = Trip::new(start, end, 125);
        assert_f32_near!(OPEN_TRIP.calculate_cost(t), 40.5);
    }
}
