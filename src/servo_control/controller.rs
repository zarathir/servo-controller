use std::{sync::Arc, thread, time::Duration};

use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Pca9685};

use crate::servo_control::Motor;

pub struct Controller {
    controller: Pca9685<I2cdev>,
}

impl Controller {
    pub fn new(path: &str, frequency: u16) -> Controller {
        let dev = I2cdev::new(path.to_string()).unwrap();
        let prescale = (25000000.0 / (4096.0 * frequency as f32) - 1.0).round() as u8;

        let mut controller = Pca9685::new(dev, Address::default()).unwrap();

        controller.set_prescale(prescale).unwrap();
        controller.enable().unwrap();

        Controller { controller }
    }

    pub fn motor_angle(&mut self, motor: &mut Motor, angle: u32) {
        self.controller
            .set_channel_on_off(
                *motor.channel(),
                0,
                calculate_duty_cycle(angle, *motor.max_angle()),
            )
            .unwrap();

        motor.set_angle(angle);
    }

    pub fn motor_angle_speed(&self, motor: &Motor, angle: u32, speed: Duration) {
        //TODO: Add atomic reference to move all motors with time constaint
        todo!("Add atomic reference to move all motors with time constaint");

        let mut current_anlge: f64 = motor.angle() as f64;
        let con = Arc::new(self);
        let mot = Arc::new(motor);

        thread::spawn(move || return);
    }
}

fn calculate_duty_cycle(angle: u32, max_angle: u32) -> u16 {
    (((angle as f32 / max_angle as f32 * 10.0) + 2.0) / 100.0 * 4096.0).round() as u16
}

pub fn map(x: i64, in_min: i64, in_max: i64, out_min: i64, out_max: i64) -> i64 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
