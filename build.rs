use std::{env, process::Command};

fn main() {
    let ros_distro = env::var("ROS_DISTRO").unwrap();

    Command::new("/bin/bash")
        .arg(format!("/opt/ros/{}/setup.bash", ros_distro))
        .status()
        .unwrap();
}
