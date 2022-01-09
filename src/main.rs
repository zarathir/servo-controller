use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::{thread, time};

struct Servo {
    device: Pca9685<I2cdev>,
    channel: Channel,
}

impl Servo {
    fn new(mut device: &Pca9685<I2cdev>, channel: Channel) -> Servo {
        Servo { device, channel }
    }

    fn set_angle(&mut self, angle: u32) {
        let duty_cycle = (((angle as f32 / 180.0 * 10.0) + 2.0) / 100.0 * 4096.0).round() as u16;

        self.device
            .set_channel_on_off(self.channel, 0, duty_cycle)
            .unwrap();
    }
}

fn set_angle(angle: u32) -> u16 {
    (((angle as f32 / 180.0 * 10.0) + 2.0) / 100.0 * 4096.0).round() as u16
}

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = Address::default();
    let mut servo_controller = Pca9685::new(dev, address).unwrap();

    servo_controller.set_prescale(100).unwrap();
    servo_controller.enable().unwrap();

    let mut motor_1 = Servo::new(servo_controller, Channel::C0);

    println!("setting duty cycle to: 90°");

    servo_controller
        .set_channel_on_off(Channel::C0, 0, set_angle(90))
        .unwrap();

    thread::sleep(time::Duration::from_millis(5000));

    println!("setting duty cycle to: 0°");

    servo_controller
        .set_channel_on_off(Channel::C0, 0, set_angle(0))
        .unwrap();

    thread::sleep(time::Duration::from_millis(5000));

    println!("setting duty cycle to: 90°");

    servo_controller
        .set_channel_on_off(Channel::C0, 0, set_angle(90))
        .unwrap();

    thread::sleep(time::Duration::from_millis(5000));

    println!("setting duty cycle to: 180°");

    servo_controller
        .set_channel_on_off(Channel::C0, 0, set_angle(180))
        .unwrap();

    thread::sleep(time::Duration::from_millis(5000));
    println!("setting duty cycle to: 90°");

    servo_controller
        .set_channel_on_off(Channel::C0, 0, set_angle(90))
        .unwrap();

    let _dev = servo_controller.destroy();
}
