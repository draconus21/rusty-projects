pub mod openplustier;
pub mod opentier;
mod tests;

trait Cost {
    // cost for distance
    fn km_cost(&self) -> f32;
    fn time_cost(&self) -> f32;
    fn trip_cost(&self) -> f32;
}
