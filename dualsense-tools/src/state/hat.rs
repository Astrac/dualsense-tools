/// Represents the direction of the hat of a Dualsense controller
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
