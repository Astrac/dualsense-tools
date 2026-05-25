use std::fmt::Display;

use dualsense_tools::state::HatDirection;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum Hat {
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

impl Display for Hat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hat::Up => f.write_str("Up"),
            Hat::UpRight => f.write_str("UpRight"),
            Hat::Right => f.write_str("Right"),
            Hat::DownRight => f.write_str("DownRight"),
            Hat::Down => f.write_str("Down"),
            Hat::DownLeft => f.write_str("DownLeft"),
            Hat::Left => f.write_str("Left"),
            Hat::UpLeft => f.write_str("UpLeft"),
            Hat::Neutral => f.write_str("Neutral"),
        }
    }
}

impl From<HatDirection> for Hat {
    fn from(val: HatDirection) -> Self {
        match val {
            HatDirection::Up => Hat::Up,
            HatDirection::UpRight => Hat::UpRight,
            HatDirection::Right => Hat::Right,
            HatDirection::DownRight => Hat::DownRight,
            HatDirection::Down => Hat::Down,
            HatDirection::DownLeft => Hat::DownLeft,
            HatDirection::Left => Hat::Left,
            HatDirection::UpLeft => Hat::UpLeft,
            HatDirection::Neutral => Hat::Neutral,
        }
    }
}
