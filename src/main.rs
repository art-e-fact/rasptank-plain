pub mod subsystems;

// use crate::subsystems::leds::setup;
use crate::subsystems::caterpillars::Caterpillars;

fn main() {
    // let mut leds = setup();
    // leds.demo();
    let mut c = Caterpillars {
        left_motor_pin: 14,
        right_motor_pin: 15,
    };
    match c.forward(3, 100) {
        Ok(duration) => println!("Run completed for {:?} seconds", duration),
        Err(e) => println!("Failure in running forward: {:?}", e),
    }
}
