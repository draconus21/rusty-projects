use crate::utils::{Trip, TripDuration};

mod tests;

enum Plan {
    Open {
        included_km: u32,
        excess_km_rate: f32,
        hour_rate: f32,
        first_day_max: f32,
        day_max: f32,
    },
    //OpenPlus {},
}

const OPEN_TRIP: Plan = Plan::Open {
    included_km: 75,
    excess_km_rate: 0.27,
    hour_rate: 13.5,
    first_day_max: 55.0,
    day_max: 50.0,
};

impl Plan {
    fn km_cost(&self, distance_km: u32) -> f32 {
        match self {
            Plan::Open {
                included_km,
                excess_km_rate,
                hour_rate: _,
                first_day_max: _,
                day_max: _,
            } => {
                if distance_km <= *included_km {
                    return 0 as f32;
                } else {
                    return (distance_km - *included_km) as f32 * excess_km_rate;
                }
            }
        }
    }
    fn time_cost(&self, duration: TripDuration) -> f32 {
        match self {
            Plan::Open {
                included_km: _,
                excess_km_rate: _,
                hour_rate,
                first_day_max,
                day_max,
            } => {
                let total_days: u32 = duration.days() + duration.weeks() * 7;
                let mut hour_cost: f32 = duration.hours() * *hour_rate;

                // Case 1: less than 24 hours
                if total_days == 0 {
                    if hour_cost < *first_day_max {
                        return hour_cost;
                    } else {
                        return *first_day_max;
                    }
                } else {
                    // cap excess hour cost to day_max
                    if hour_cost > *day_max {
                        hour_cost = *day_max;
                    }

                    // Case 2: 1 day < duration < 2 days
                    if total_days == 1 {
                        return *first_day_max + hour_cost;
                    } else {
                        //Case 3: duration > 2 days
                        return *first_day_max + (total_days - 1) as f32 * *day_max + hour_cost;
                    }
                }
            }
        }
    }

    fn calculate_cost(&self, trip: Trip) -> f32 {
        let distance_cost = self.km_cost(*trip.distance());
        let time_cost = self.time_cost(trip.trip_time());
        distance_cost + time_cost
    }
}
