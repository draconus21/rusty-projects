mod tests;

use super::utils::{Time, TripDuration};
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct OpenTrip {
    start_time: Time,
    end_time: Time,
    distance: u32,
}
impl OpenTrip {
    fn validate_trip_time(start_time: Time, end_time: Time) {
        if start_time >= end_time {
            panic!(
                "Trip start time {:?} cannot be after end time {:?}",
                start_time, end_time
            )
        }
    }

    pub fn new(start_time: Time, end_time: Time, distance: u32) -> Self {
        OpenTrip::validate_trip_time(start_time, end_time);
        Self {
            start_time,
            end_time,
            distance,
        }
    }

    pub fn start_time(&self) -> &Time {
        &self.start_time
    }
    pub fn set_start_time(&self, start_time: Time) -> Self {
        OpenTrip::validate_trip_time(start_time, self.end_time);
        Self {
            start_time: start_time,
            end_time: self.end_time,
            distance: self.distance,
        }
    }
    pub fn end_time(&self) -> &Time {
        &self.end_time
    }
    pub fn set_end_time(&self, end_time: Time) -> Self {
        OpenTrip::validate_trip_time(self.start_time, end_time);
        Self {
            start_time: self.start_time,
            end_time: end_time,
            distance: self.distance,
        }
    }
    pub fn distance(&self) -> &u32 {
        &self.distance
    }
    pub fn set_distance(&self, distance: u32) -> Self {
        Self {
            start_time: self.start_time,
            end_time: self.end_time,
            distance: distance,
        }
    }

    fn trip_time(&self) -> TripDuration {
        TripDuration::from_times(self.start_time(), self.end_time())
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
        if self.distance < 75 {
            0 as f32
        } else {
            (self.distance - 75) as f32 * 0.27 as f32
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
        writeln!(f, "From         : {}", self.start_time)?;
        writeln!(f, "To           : {}", self.end_time)?;
        writeln!(f, "Duration     : {}", self.trip_time())?;
        writeln!(f, "Duration cost: {}", self.time_cost())?;
        writeln!(f, "Distance     : {} km", self.distance)?;
        writeln!(f, "Distance cost: {}", self.km_cost())?;
        writeln!(f, "Total        : {}", self.trip_cost())?;
        writeln!(f, "--------------------------------------")
    }
}
