use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
pub struct ServoController {
    controller: Pca9685<I2cdev>,
}

pub struct ServoMotor {
    channel: Channel,
    angle: u32,
}

#[derive(Debug)]
pub enum ChannelError {
    ChannelError(String),
}

impl ServoController {
    pub fn new(path: &str) -> ServoController {
        let dev = I2cdev::new(path.to_string()).unwrap();

        let controller = Pca9685::new(dev, Address::default()).unwrap();

        ServoController { controller }
    }

    pub fn init(&mut self, freq: u16) {
        let prescale = (25000000.0 / (4096.0 * freq as f32) - 1.0).round() as u8;
        self.controller.set_prescale(prescale).unwrap();
        self.controller.enable().unwrap();
    }

    pub fn set_motor_angle(&mut self, motor: &mut ServoMotor, angle: u32) {
        self.controller
            .set_channel_on_off(motor.channel, 0, calculate_duty_cycle(angle))
            .unwrap();

        motor.set_angle(angle);
    }
}

impl ServoMotor {
    pub fn new(channel_number: u8, angle: u32) -> Result<ServoMotor, ChannelError> {
        let channel = match channel_number {
            0 => Channel::C0,
            1 => Channel::C1,
            2 => Channel::C2,
            3 => Channel::C3,
            4 => Channel::C4,
            5 => Channel::C5,
            6 => Channel::C6,
            7 => Channel::C7,
            8 => Channel::C8,
            9 => Channel::C9,
            10 => Channel::C10,
            11 => Channel::C11,
            12 => Channel::C12,
            13 => Channel::C13,
            14 => Channel::C14,
            15 => Channel::C15,
            _other => {
                return Err(ChannelError::ChannelError(
                    "Channel number does not exist. Please choose one between 0-15".to_string(),
                ))
            }
        };

        Ok(ServoMotor { channel, angle })
    }

    pub fn set_angle(&mut self, angle: u32) {
        self.angle = angle;
    }

    pub fn get_angle(&mut self) -> u32 {
        self.angle
    }
}

fn calculate_duty_cycle(angle: u32) -> u16 {
    (((angle as f32 / 180.0 * 10.0) + 2.0) / 100.0 * 4096.0).round() as u16
}
