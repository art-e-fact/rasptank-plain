use std::thread;
use std::time::Duration;

pub mod subsystems;

use crate::subsystems::leds::setup as led_setup;
use crate::subsystems::caterpillars::setup as cat_setup;

fn main() {
    let mut leds = led_setup();
    leds.demo();
    let mut caterpillars = cat_setup(
        14, 15, 4,
        27, 18, 17,
        );
    match caterpillars.as_mut().expect("Unabled to access caterpillars").forward(0.5) {
        Ok(state) => println!("{:?}", state),
        Err(e) => println!("Failure in running forward: {:?}", e),
    }
    thread::sleep(Duration::from_secs(3));
    match caterpillars.as_mut().expect("Unabled to access caterpillars").stop() {
        Ok(state) => println!("{:?}", state),
        Err(e) => println!("Failure in stopping: {:?}", e),
    }
}
