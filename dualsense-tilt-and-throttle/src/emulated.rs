use std::fmt::Display;

use dualsense_tools::state::HatDirection;

use crate::emulated_axis_value::EmulatedAxisValue;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct EmulatedAxes {
    pub x: EmulatedAxisValue,
    pub y: EmulatedAxisValue,
    pub rx: EmulatedAxisValue,
    pub ry: EmulatedAxisValue,
    pub throttle: EmulatedAxisValue,
    pub pitch: EmulatedAxisValue,
    pub roll: EmulatedAxisValue,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum EmulatedHat {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    #[default]
    Neutral,
}

impl Display for EmulatedHat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmulatedHat::Up => f.write_str("Up"),
            EmulatedHat::UpRight => f.write_str("UpRight"),
            EmulatedHat::Right => f.write_str("Right"),
            EmulatedHat::DownRight => f.write_str("DownRight"),
            EmulatedHat::Down => f.write_str("Down"),
            EmulatedHat::DownLeft => f.write_str("DownLeft"),
            EmulatedHat::Left => f.write_str("Left"),
            EmulatedHat::UpLeft => f.write_str("UpLeft"),
            EmulatedHat::Neutral => f.write_str("Neutral"),
        }
    }
}

impl From<HatDirection> for EmulatedHat {
    fn from(val: HatDirection) -> Self {
        match val {
            HatDirection::Up => EmulatedHat::Up,
            HatDirection::UpRight => EmulatedHat::UpRight,
            HatDirection::Right => EmulatedHat::Right,
            HatDirection::DownRight => EmulatedHat::DownRight,
            HatDirection::Down => EmulatedHat::Down,
            HatDirection::DownLeft => EmulatedHat::DownLeft,
            HatDirection::Left => EmulatedHat::Left,
            HatDirection::UpLeft => EmulatedHat::UpLeft,
            HatDirection::Neutral => EmulatedHat::Neutral,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct EmulatedGamepad {
    pub axes: EmulatedAxes,
    pub hat: EmulatedHat,
    pub buttons: [bool; 13],
    pub is_tilt_enabled: bool,
}
