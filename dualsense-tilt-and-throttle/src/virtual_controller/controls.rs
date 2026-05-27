use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Hash, Eq)]
pub enum AxisId {
    X,
    Y,
    Z,
    RX,
    RY,
    RZ,
    Pitch,
    Roll,
    Throttle,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Hash, Eq)]
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
    #[serde(rename = "Option")]
    Opt,
    Share,
    Mic,
    TouchClick,
    Ps,
}
