#[cfg(test)]

mod test {

    use crate::utils::{Time, Trip};

    use super::super::{
        LONG_DISTANCE_HIGH, LONG_DISTANCE_LOW, OPEN_PLUS_TRIP, OPEN_TRIP, VALUE_TRIP,
    };
    use assert_float_eq::*;

    #[test]
    fn test_km_cost() {
        assert_eq!(OPEN_TRIP.km_cost(50), 0.0);
        assert_f32_near!(OPEN_PLUS_TRIP.km_cost(50), 12.5);
        assert_f32_near!(VALUE_TRIP.km_cost(50), 22.5);
        assert_f32_near!(LONG_DISTANCE_HIGH.km_cost(50), 11.5);
        assert_f32_near!(LONG_DISTANCE_LOW.km_cost(50), 11.5);

        assert_f32_near!(OPEN_TRIP.km_cost(175), 27.0);
        assert_f32_near!(OPEN_PLUS_TRIP.km_cost(175), 40.0);
        assert_f32_near!(VALUE_TRIP.km_cost(175), 62.5);
        assert_f32_near!(LONG_DISTANCE_HIGH.km_cost(175), 40.25);
        assert_f32_near!(LONG_DISTANCE_LOW.km_cost(175), 40.25);

        assert_f32_near!(LONG_DISTANCE_HIGH.km_cost(400), 84.0);
        assert_f32_near!(LONG_DISTANCE_LOW.km_cost(400), 84.0);
    }

    #[test]
    fn test_time_cost() {
        let start = Time::new(16, 32, 3, 8, 2025);
        let end = Time::new(18, 32, 3, 8, 2025);

        // 2 hr Trip
        let mut t = Trip::new(start, end, 10);
        assert_f32_near!(OPEN_TRIP.time_cost(&t.trip_time()), 27.0);
        assert_f32_near!(OPEN_PLUS_TRIP.time_cost(&t.trip_time()), 13.7);
        assert_f32_near!(VALUE_TRIP.time_cost(&t.trip_time()), 7.2);
        assert_f32_near!(LONG_DISTANCE_HIGH.time_cost(&t.trip_time()), 50.0);
        assert_f32_near!(LONG_DISTANCE_LOW.time_cost(&t.trip_time()), 40.0);

        // 5 hr Trip
        t = Trip::new(start, Time::new(21, 32, 3, 8, 2025), 10);
        assert_f32_near!(OPEN_TRIP.time_cost(&t.trip_time()), 55.0);
        assert_f32_near!(OPEN_PLUS_TRIP.time_cost(&t.trip_time()), 34.25);
        assert_f32_near!(VALUE_TRIP.time_cost(&t.trip_time()), 18.0);
        assert_f32_near!(LONG_DISTANCE_HIGH.time_cost(&t.trip_time()), 50.0);
        assert_f32_near!(LONG_DISTANCE_LOW.time_cost(&t.trip_time()), 40.0);

        // 12 hr Trip
        t = Trip::new(start, Time::new(4, 32, 4, 8, 2025), 10);
        assert_f32_near!(OPEN_TRIP.time_cost(&t.trip_time()), 55.0);
        assert_f32_near!(OPEN_PLUS_TRIP.time_cost(&t.trip_time()), 50.0);
        assert_f32_near!(VALUE_TRIP.time_cost(&t.trip_time()), 30.0);
        assert_f32_near!(LONG_DISTANCE_HIGH.time_cost(&t.trip_time()), 50.0);
        assert_f32_near!(LONG_DISTANCE_LOW.time_cost(&t.trip_time()), 40.0);

        // 2-day Trip
        t = Trip::new(start, Time::new(16, 32, 5, 8, 2025), 10);
        assert_f32_near!(OPEN_TRIP.time_cost(&t.trip_time()), 105.0);
        assert_f32_near!(OPEN_PLUS_TRIP.time_cost(&t.trip_time()), 85.0);
        assert_f32_near!(VALUE_TRIP.time_cost(&t.trip_time()), 60.0);
        assert_f32_near!(LONG_DISTANCE_HIGH.time_cost(&t.trip_time()), 92.0);
        assert_f32_near!(LONG_DISTANCE_LOW.time_cost(&t.trip_time()), 69.95);

        // 5d 8h day Trip
        t = Trip::new(start, Time::new(0, 32, 9, 8, 2025), 10);
        assert_f32_near!(OPEN_TRIP.time_cost(&t.trip_time()), 305.0);
        assert_f32_near!(OPEN_PLUS_TRIP.time_cost(&t.trip_time()), 225.0);
        assert_f32_near!(VALUE_TRIP.time_cost(&t.trip_time()), 178.8);
        assert_f32_near!(LONG_DISTANCE_HIGH.time_cost(&t.trip_time()), 260.0);
        assert_f32_near!(LONG_DISTANCE_LOW.time_cost(&t.trip_time()), 189.75);

        // 8 day 4h Trip
        t = Trip::new(start, Time::new(20, 32, 11, 8, 2025), 10);
        assert_f32_near!(LONG_DISTANCE_HIGH.time_cost(&t.trip_time()), 304.0);
        assert_f32_near!(LONG_DISTANCE_LOW.time_cost(&t.trip_time()), 244.9);
    }
    #[test]
    fn test_total_cost() {
        let start = Time::new(17, 5, 4, 8, 2025);
        let end = Time::new(21, 25, 12, 8, 2025);

        let t = Trip::new(start, end, 422);
        assert_f32_near!(OPEN_PLUS_TRIP.calculate_cost(&t), 419.02335);
        assert_f32_near!(VALUE_TRIP.calculate_cost(&t), 397.14);
    }
}
