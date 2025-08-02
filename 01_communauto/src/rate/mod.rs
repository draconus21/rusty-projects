mod tests;

use super::utils::Time;

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
}

trait Cost {
    // cost for distance
    fn km_cost(&self) -> f32;
    //fn trip_cost(&self, start_time: Time, end_time: Time, distance: i16) -> f32;
    //fn time_cost(&self, start_time: Time, end_time: Time) -> f32
}

impl Cost for OpenTrip {
    fn km_cost(&self) -> f32 {
        if self.distance < 75 {
            0 as f32
        } else {
            (self.distance - 75) as f32 * 0.27 as f32
        }
    }
}
