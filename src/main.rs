use std::{io::stdin, num::ParseIntError, thread, time};

mod servo_control;

use crate::servo_control::{Controller, Motor};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //TODO: Add JointState ROS subscriber

    let mut controller = Controller::new("/dev/i2c-1", 50);

    let mut motor_1 = Motor::new("l0", 0, 180).unwrap();
    let mut motor_2 = Motor::new("l1", 1, 270).unwrap();
    let start_angle_1 = motor_1.max_angle() / 2;
    let start_angle_2 = motor_2.max_angle() / 2;
    controller.motor_angle(&mut motor_1, start_angle_1);
    controller.motor_angle(&mut motor_2, start_angle_2);

    let handle = tokio::task::spawn_blocking(move || loop {
        println!("Where do you want to move the servo?");

        match get_input() {
            Ok(angle) => {
                controller.motor_angle(&mut motor_1, angle);
                controller.motor_angle(&mut motor_2, angle);
                println!(
                    "Moving servo to {}°, {} rad",
                    angle,
                    (angle as f64).to_radians()
                );
                sleep(1);
            }
            Err(err) => eprintln!("Did not enter a valid number {}", &err),
        };
    });

    handle.await?;

    Ok(())
}
