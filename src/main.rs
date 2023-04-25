use rs_ws281x::ControllerBuilder;

fn main() {
    // Configuration based on Adeept's Python code
    let controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            ChannelBuilder::new()
            .pin(12)
            .count(16)
            .strip_type(StripType::Ws2812)
            .brightness(255)
            .build()
        )
        .build();

    // get the strand of LEDs on channel 1
    let leds = controller.leds_mut(0);
    // set the first LED to white (with the configured
    // strip above, this is BGRW)
    leds[0] = [255, 255, 255, 0]

    // render it to the strand
    controller.render();
}
