use chrono::Local;
use futures::{future, stream::StreamExt};
use std::{io::stdin, num::ParseIntError, thread, time};

mod servo_control;

use r2r::QosProfile;

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
    //TODO: Move ROS part to own crate
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "servo-controller", "")?;
    let qos_profile = QosProfile::default();
    let sub =
        node.subscribe::<r2r::sensor_msgs::msg::JointState>("/joint_states", qos_profile.clone())?;

    let publisher = node.create_publisher::<r2r::sensor_msgs::msg::JointState>(
        "/joint_states",
        qos_profile.clone(),
    )?;

    let mut controller = Controller::new("/dev/i2c-1", 50);

    let mut motor_1 = Motor::new("l0", 0, 180).unwrap();
    let mut motor_2 = Motor::new("l1", 1, 270).unwrap();
    let start_angle_1 = motor_1.max_angle() / 2;
    let start_angle_2 = motor_2.max_angle() / 2;
    controller.motor_angle(&mut motor_1, start_angle_1);
    controller.motor_angle(&mut motor_2, start_angle_2);

    let mut frame_id: u64 = 0;

    let handle = tokio::task::spawn_blocking(move || loop {
        println!("Where do you want to move the servo?");

        let time = Local::now();
        let stamp = r2r::builtin_interfaces::msg::Time {
            sec: time.timestamp().try_into().unwrap(),
            nanosec: time.timestamp_subsec_nanos(),
        };

        node.spin_once(std::time::Duration::from_millis(100));

        match get_input() {
            Ok(angle) => {
                controller.motor_angle(&mut motor_1, angle);
                controller.motor_angle(&mut motor_2, angle);
                let pub_msg = r2r::sensor_msgs::msg::JointState {
                    header: r2r::std_msgs::msg::Header {
                        frame_id: frame_id.to_string(),
                        stamp,
                    },
                    name: vec![motor_1.name().to_string(), motor_2.name().to_string()],
                    position: vec![
                        (motor_1.angle() as f64).to_radians(),
                        (motor_2.angle() as f64).to_radians(),
                    ],
                    velocity: Vec::new(),
                    effort: Vec::new(),
                };
                println!(
                    "Moving servo to {}°, {} rad",
                    angle,
                    (angle as f64).to_radians()
                );
                publisher.publish(&pub_msg).unwrap();
                sleep(1);
            }
            Err(err) => eprintln!("Did not enter a valid number {}", &err),
        };

        frame_id += 1;
    });

    sub.for_each(|msg| {
        println!(
            "received: new msg.name: {:?}, msg.position: {:?}",
            msg.name, msg.position
        );
        future::ready(())
    })
    .await;

    handle.await?;

    Ok(())
}
