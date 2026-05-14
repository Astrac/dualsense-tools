#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum HatDirection {
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

impl From<u8> for HatDirection {
    fn from(value: u8) -> Self {
        match value & 0b00001111 {
            0 => Self::Up,
            1 => Self::UpRight,
            2 => Self::Right,
            3 => Self::DownRight,
            4 => Self::Down,
            5 => Self::DownLeft,
            6 => Self::Left,
            7 => Self::UpLeft,
            8 => Self::Neutral,
            _ => unreachable!(),
        }
    }
}
