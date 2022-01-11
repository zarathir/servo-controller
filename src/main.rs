use servo_control::{ServoController, ServoMotor};
use std::{io::stdin, num::ParseIntError, thread, time};

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
    let mut controller = ServoController::new("/dev/i2c-1");
    controller.init(60);

    let mut motor_1 = ServoMotor::new(0, 90).unwrap();
    let start_angle = motor_1.get_angle();
    controller.set_motor_angle(&mut motor_1, start_angle);

    loop {
        println!("Where do you want to move the servo?");

        match get_input() {
            Ok(angle) => {
                controller.set_motor_angle(&mut motor_1, angle);
                println!("Moving servo to {}", angle);
                sleep(5);
            }
            Err(err) => eprintln!("Did not enter a valid number {}", &err),
        };
    }
}
