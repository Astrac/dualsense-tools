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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AxisId {
    LX,
    LY,
    LZ,
    RX,
    RY,
    RZ,
}

pub enum ControlId {
    Axis(AxisId),
    Button(ButtonId),
    Accelerometer,
    Gyroscope,
    TiltEstimator
}
