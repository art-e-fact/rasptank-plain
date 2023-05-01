use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, Error, OutputPin};

pub struct Caterpillars {
    pub left_motor_pin1: OutputPin,
    pub left_motor_pin2: OutputPin,
    pub left_motor_pin_en: OutputPin,
    pub right_motor_pin1: OutputPin,
    pub right_motor_pin2: OutputPin,
    pub right_motor_pin_en: OutputPin,
}

impl Caterpillars {
    pub fn forward(&mut self, seconds: u64, speed: f64) -> Result<u64, Error> {
        self.left_motor_pin1.set_high();
        self.left_motor_pin2.set_low();
        self.left_motor_pin_en.set_pwm_frequency(1000.0, speed);
        self.right_motor_pin1.set_low();
        self.right_motor_pin2.set_high();
        self.right_motor_pin_en.set_pwm_frequency(1000.0, speed);
        thread::sleep(Duration::from_secs(seconds));
        self.left_motor_pin1.set_low();
        self.left_motor_pin2.set_low();
        self.left_motor_pin_en.clear_pwm();
        self.right_motor_pin1.set_low();
        self.right_motor_pin2.set_low();
        self.right_motor_pin_en.clear_pwm();
        Ok(seconds)
    }
}

pub fn setup(lmp1n: u8, lmp2n: u8, lmpenn: u8, rmp1n: u8, rmp2n: u8, rmpenn: u8) -> Result<Caterpillars, Error> {
    let gpio = Gpio::new()?;
    let mut lmp1 = gpio.get(lmp1n)?.into_output();
    lmp1.set_low();
    let mut lmp2 = gpio.get(lmp2n)?.into_output();
    lmp2.set_low();
    let mut lmpen = gpio.get(lmpenn)?.into_output();
    lmpen.clear_pwm();
    let mut rmp1 = gpio.get(rmp1n)?.into_output();
    rmp1.set_low();
    let mut rmp2 = gpio.get(rmp2n)?.into_output();
    rmp2.set_low();
    let mut rmpen = gpio.get(rmpenn)?.into_output();
    rmpen.clear_pwm();
    Ok(
        Caterpillars {
            left_motor_pin1: lmp1,
            left_motor_pin2: lmp2,
            left_motor_pin_en: lmpen,
            right_motor_pin1: rmp1,
            right_motor_pin2: rmp2,
            right_motor_pin_en: rmpen,
        }
      )
}
