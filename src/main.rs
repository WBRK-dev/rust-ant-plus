use std::{thread::sleep, time::Duration};

use sticks::Driver;

use crate::sticks::garmin_stick_3::GarminStick3;

mod constants;
mod events;
mod sticks;
mod messages;

fn main() {
    let mut stick = GarminStick3::new();

    println!("Is Garmin Stick 3 present? {}", stick.is_present());

    if stick.open() {
        println!("Garmin Stick 3 opened successfully.");
    } else {
        println!("Failed to open Garmin Stick 3.");
    }

    stick.on("shutdown", |_| {
        println!("Shutdown event triggered!");
    });

    sleep(Duration::from_secs(1));

    stick.close();    
}
