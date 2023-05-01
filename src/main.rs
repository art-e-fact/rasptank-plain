pub mod subsystems;

use crate::subsystems::leds::setup as led_setup;
use crate::subsystems::caterpillars::setup as cat_setup;

fn main() {
    let mut leds = led_setup();
    leds.demo();
    let caterpillars = cat_setup(
        14, 15, 4,
        27, 18, 17,
        );
    match caterpillars.expect("Unabled to access caterpillars").forward(3, 0.5) {
        Ok(duration) => println!("Forward run for {:?} seconds", duration),
        Err(e) => println!("Failure in running forward: {:?}", e),
    }
}
