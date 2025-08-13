mod plan;
mod rate;
mod utils;

use crate::rate::openplustier::OpenPlusTrip;
use crate::rate::opentier::OpenTrip;
use crate::utils::Time;
use std::io;

fn get_input(prompt: &str) -> String {
    println!("{}: ", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input.trim().to_string()
}
fn get_u32(prompt: &str) -> u32 {
    loop {
        let day = get_input(prompt);
        match day.parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid day"),
        };
    }
}

fn get_time(prompt: &str) -> Time {
    println!("{}", prompt);
    Time::new(
        get_u32("hours"),
        get_u32("minutes"),
        get_u32("day"),
        get_u32("month"),
        get_u32("year"),
    )
}
fn main() {
    //let start: Time = get_time("Trip start time");
    //let end: Time = get_time("Trip end time");
    //let dist: u32 = get_u32("Trip distance");

    let start = Time::now();
    let end = Time::tomorrow();
    let dist = 175;
    println!("{}", OpenTrip::new(start, end, dist));
    println!("{}", OpenPlusTrip::new(start, end, dist));
}
