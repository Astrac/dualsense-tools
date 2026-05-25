#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Identifies the buttons of a Dualsense controller
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    #[cfg_attr(feature = "serde", serde(rename = "Option"))]
    Opt,
    Share,
    Mic,
    TouchClick,
    Ps
}

/// Identifies the axes of a Dualsense controller
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum AxisId {
    LX,
    LY,
    LZ,
    RX,
    RY,
    RZ,
}
