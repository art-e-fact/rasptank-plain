pub mod subsystems;

use crate::subsystems::leds::setup;

fn main() {
    let mut leds = setup();
    leds.demo();
}
