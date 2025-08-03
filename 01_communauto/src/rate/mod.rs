mod tests;

use super::utils::{Time, Trip, TripDuration};
use std::fmt::Display;

pub struct OpenTrip {
    trip: Trip,
}
impl OpenTrip {
    pub fn new(start_time: Time, end_time: Time, distance: u32) -> Self {
        Self {
            trip: Trip::new(start_time, end_time, distance),
        }
    }

    // Shadow Trip functions
    pub fn set_distance(&self, distance: u32) -> Self {
        Self {
            trip: self.trip.set_distance(distance),
        }
    }
    pub fn distance(&self) -> &u32 {
        self.trip.distance()
    }
    pub fn start_time(&self) -> &Time {
        self.trip.start_time()
    }
    pub fn end_time(&self) -> &Time {
        self.trip.end_time()
    }
    pub fn trip_time(&self) -> TripDuration {
        self.trip.trip_time()
    }
}
trait Cost {
    // cost for distance
    fn km_cost(&self) -> f32;
    fn time_cost(&self) -> f32;
    fn trip_cost(&self) -> f32;
}

impl Cost for OpenTrip {
    fn trip_cost(&self) -> f32 {
        self.km_cost() + self.time_cost()
    }
    fn km_cost(&self) -> f32 {
        if *self.distance() < 75 {
            0 as f32
        } else {
            (*self.distance() - 75) as f32 * 0.27 as f32
        }
    }

    fn time_cost(&self) -> f32 {
        let duration: TripDuration = self.trip_time();

        let total_days: u32 = duration.days() + duration.weeks() * 7;
        let mut hour_cost: f32 = duration.hours() * 13.5;

        // Case 1: less that 24 hours
        if total_days == 0 {
            // Min of hourly rate vs day rate
            if hour_cost < 55.0 {
                return hour_cost;
            } else {
                return 55.0;
            }
        }

        // cap hour cost to 50$ for subsequent days
        if hour_cost > 50.0 {
            hour_cost = 50.0
        }
        // Case 2: 24 hrs < duration < 48 hrs
        if total_days == 1 {
            return 55.0 + hour_cost;
        }

        // Case 3: > 48 hours
        55.0 + 50.0 * (total_days - 1) as f32 + hour_cost
    }
}

impl Display for OpenTrip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--------------Plan: Open--------------")?;
        writeln!(f, "{}", self.trip)?;
        writeln!(f, "Duration cost: {}", self.time_cost())?;
        writeln!(f, "Distance cost: {}", self.km_cost())?;
        writeln!(f, "Total        : {}", self.trip_cost())?;
        write!(f, "--------------------------------------")
    }
}
