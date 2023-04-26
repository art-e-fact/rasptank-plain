use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::Error;

pub struct Caterpillars {
    pub left_motor_pin: u8,
    pub right_motor_pin: u8,
}

impl Caterpillars {
    pub fn forward(&mut self, seconds: u64, _speed: u64) -> Result<u64, Error> {
        let gpio = Gpio::new()?;
        let mut lm = gpio.get(self.left_motor_pin)?.into_output();
        let mut rm = gpio.get(self.right_motor_pin)?.into_output();
        lm.set_high();
        rm.set_high();
        thread::sleep(Duration::from_secs(seconds));
        rm.set_low();
        lm.set_low();
        Ok(seconds)
    }
}
