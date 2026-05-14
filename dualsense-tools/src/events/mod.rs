mod determinator;
mod determinator_config;
mod event_bag;

use crate::{
    Tilt,
    state::{Accel, Gyro, HatDirection},
};

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonEventKind {
    Pressed,
    Released,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    AccelUpdated(Accel<i16>),
    AxisChanged(AxisId, u8),
    Button(ButtonId, ButtonEventKind),
    GyroUpdated(Gyro<i16>),
    HatDirectionChanged(HatDirection),
    TiltEstimateUpdted(Tilt),
}
