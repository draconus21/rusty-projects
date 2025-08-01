mod tests;
pub struct RatePlan {
    name: String,
}
//trait Rate {
//    fn trip_cost(start_date: i16, end_date: i16, distance: i16) -> f16;
//}

impl RatePlan {
    pub fn new(name: String) -> Self {
        Self { name: name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

//impl Rate for RatePlan {}
