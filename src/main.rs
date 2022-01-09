use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::{thread, time};

fn set_angle(angle: u32) -> u16 {
    (((angle as f32 / 180.0 * 10.0) + 2.0) / 100.0 * 4096.0).round() as u16
}

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();

    pwm.set_prescale(100).unwrap();
    pwm.enable().unwrap();

    println!("setting duty cycle to: 90°");

    pwm.set_channel_on_off(Channel::C0, 0, set_angle(90))
        .unwrap();

    thread::sleep(time::Duration::from_millis(5000));

    println!("setting duty cycle to: 0°");

    pwm.set_channel_on_off(Channel::C0, 0, set_angle(0))
        .unwrap();

    thread::sleep(time::Duration::from_millis(5000));

    println!("setting duty cycle to: 90°");

    pwm.set_channel_on_off(Channel::C0, 0, set_angle(90))
        .unwrap();

    thread::sleep(time::Duration::from_millis(5000));

    println!("setting duty cycle to: 180°");

    pwm.set_channel_on_off(Channel::C0, 0, set_angle(180))
        .unwrap();

    thread::sleep(time::Duration::from_millis(5000));

    println!("setting duty cycle to: 90°");

    pwm.set_channel_on_off(Channel::C0, 0, set_angle(90))
        .unwrap();

    thread::sleep(time::Duration::from_millis(5000));

    let _dev = pwm.destroy();
}
