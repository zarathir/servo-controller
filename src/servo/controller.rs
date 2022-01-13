use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Pca9685};

use crate::servo::Motor;

pub struct Controller {
    controller: Pca9685<I2cdev>,
}

impl Controller {
    pub fn new(path: &str) -> Controller {
        let dev = I2cdev::new(path.to_string()).unwrap();

        let controller = Pca9685::new(dev, Address::default()).unwrap();

        Controller { controller }
    }

    pub fn init(&mut self, freq: u16) {
        let prescale = (25000000.0 / (4096.0 * freq as f32) - 1.0).round() as u8;
        self.controller.set_prescale(prescale).unwrap();
        self.controller.enable().unwrap();
    }

    pub fn set_motor_angle(&mut self, motor: &mut Motor, angle: u32) {
        let max_angle = motor.get_max_angle();
        self.controller
            .set_channel_on_off(
                motor.get_channel(),
                0,
                calculate_duty_cycle(angle, max_angle),
            )
            .unwrap();

        motor.set_angle(angle);
    }
}

fn calculate_duty_cycle(angle: u32, max_angle: u32) -> u16 {
    (((angle as f32 / max_angle as f32 * 10.0) + 2.0) / 100.0 * 4096.0).round() as u16
}

pub fn map(x: i64, in_min: i64, in_max: i64, out_min: i64, out_max: i64) -> i64 {
    return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}
