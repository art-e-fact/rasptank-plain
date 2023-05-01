pub mod subsystems;

// use crate::subsystems::leds::setup;
use crate::subsystems::caterpillars::setup;

fn main() {
    // let mut leds = setup();
    // leds.demo();
    let caterpillars = setup(
        4, 14, 15,
        17, 27, 18,
        );
    match caterpillars.expect("Unabled to access caterpillars").forward(3, 0.5) {
        Ok(duration) => println!("Forward run for {:?} seconds", duration),
        Err(e) => println!("Failure in running forward: {:?}", e),
    }
}
