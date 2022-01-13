use pwm_pca9685::Channel;

pub struct Motor {
    channel: Channel,
    angle: u32,
    max_angle: u32,
}

#[derive(Debug)]
pub enum ChannelError {
    ChannelError(String),
}

impl Motor {
    pub fn new(channel_number: u8, max_angle: u32) -> Result<Motor, ChannelError> {
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

        Ok(Motor {
            channel,
            angle: 0,
            max_angle,
        })
    }

    pub fn set_angle(&mut self, angle: u32) {
        self.angle = angle;
    }

    pub fn get_angle(&mut self) -> u32 {
        self.angle
    }

    pub fn get_channel(&mut self) -> Channel {
        self.channel
    }

    pub fn get_max_angle(&mut self) -> u32 {
        self.max_angle
    }
}
