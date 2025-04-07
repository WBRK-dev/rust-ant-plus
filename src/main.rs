use devices::garmin_stick_3::GarminStick3;

mod ant;
mod constants;
mod devices;
mod messages;

fn main() {
    let stick= GarminStick3::new();

    if !stick.is_present() {
        println!("Garmin Stick 3 is not present.");
        return;
    }

    if !stick.open() {
        println!("Failed to open Garmin Stick 3.");
        return;
    }
}
