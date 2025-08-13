#[cfg(test)]
mod tests {

    use crate::utils::{Time, Trip};

    use super::super::OPEN_TRIP;
    use assert_float_eq::*;

    #[test]
    fn test_km_cost() {
        assert_eq!(OPEN_TRIP.km_cost(50), 0.0);
        assert_f32_near!(OPEN_TRIP.km_cost(175), 27.0);
    }

    #[test]
    fn test_time_cost() {
        let start = Time::new(16, 32, 3, 8, 2025);
        let end = Time::new(18, 32, 3, 8, 2025);

        // 2 hr OpenTrip
        let mut t = Trip::new(start, end, 10);
        assert_f32_near!(OPEN_TRIP.time_cost(t.trip_time()), 27.0);

        // 5 hr OpenTrip
        t = Trip::new(start, Time::new(21, 32, 3, 8, 2025), 10);
        assert_f32_near!(OPEN_TRIP.time_cost(t.trip_time()), 55.0);

        // 12 hr OpenTrip
        t = Trip::new(start, Time::new(4, 32, 4, 8, 2025), 10);
        assert_f32_near!(OPEN_TRIP.time_cost(t.trip_time()), 55.0);

        // 2-day OpenTrip
        t = Trip::new(start, Time::new(16, 32, 5, 8, 2025), 10);
        assert_f32_near!(OPEN_TRIP.time_cost(t.trip_time()), 105.0);

        // 5.5 day OpenTrip
        t = Trip::new(start, Time::new(4, 32, 9, 8, 2025), 10);
        assert_f32_near!(OPEN_TRIP.time_cost(t.trip_time()), 305.0);
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
