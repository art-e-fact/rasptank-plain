use std::{
    thread,
    time::Duration,
};

use rs_ws281x::{ControllerBuilder, StripType, ChannelBuilder};

fn main() {
    // Configuration based on Adeept's Python code
    let build_result = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
            .pin(12)
            .count(16)
            .strip_type(StripType::Ws2812)
            .brightness(255)
            .build()
        )
        .build();

    let mut controller = match build_result {
        Ok(c) => c,
        Err(error) => panic!("Problem interfacing with the LED devices: {:?}", error),
    };

    // get the strand of LEDs on channel 1
    let leds = controller.leds_mut(0);
    // set the first LED to white (with the configured
    // strip above, this is BGRW)
    leds[0] = [255, 255, 255, 0];

    // render it to the strand
    let render_result = controller.render();

    match render_result {
        Ok(_) => thread::sleep(Duration::from_secs(5)),
        Err(error) => panic!("Problem tearing down: {:?}", error),
    }
}
