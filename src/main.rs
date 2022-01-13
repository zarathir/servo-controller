use std::{io::stdin, num::ParseIntError, thread, time};

mod servo;
use crate::servo::{Controller, Motor};

fn sleep(time: u64) {
    thread::sleep(time::Duration::from_millis(time * 1000));
}

fn get_input() -> Result<u32, ParseIntError> {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");

    match input.trim().parse::<u32>() {
        Ok(angle) => Ok(angle),
        Err(err) => Err(err),
    }
}

fn main() {
    let mut controller = Controller::new("/dev/i2c-1");
    controller.init(50);

    let mut motor_1 = Motor::new(0, 180).unwrap();
    let mut motor_2 = Motor::new(1, 270).unwrap();
    let start_angle_1 = motor_1.get_max_angle() / 2;
    let start_angle_2 = motor_2.get_max_angle() / 2;
    controller.set_motor_angle(&mut motor_1, start_angle_1);
    controller.set_motor_angle(&mut motor_2, start_angle_2);

    loop {
        println!("Where do you want to move the servo?");

        match get_input() {
            Ok(angle) => {
                controller.set_motor_angle(&mut motor_1, angle);
                controller.set_motor_angle(&mut motor_2, angle);
                println!("Moving servo to {}", angle);
                sleep(2);
            }
            Err(err) => eprintln!("Did not enter a valid number {}", &err),
        };
    }
}
