/// Identifies the buttons of a Dualsense controller
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonId {
    Cross,
    Square,
    Circle,
    Triangle,
    L1,
    R1,
    L2,
    R2,
    L3,
    R3,
    Opt,
    Share,
    Mic,
    TouchClick,
}

/// Identifies the axes of a Dualsense controller
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AxisId {
    LX,
    LY,
    LZ,
    RX,
    RY,
    RZ,
}
