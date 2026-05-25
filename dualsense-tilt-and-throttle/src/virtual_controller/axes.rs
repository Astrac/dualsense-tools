use crate::virtual_controller::AxisValue;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct Axes {
    pub x: AxisValue,
    pub y: AxisValue,
    pub rx: AxisValue,
    pub ry: AxisValue,
    pub z: AxisValue,
    pub rz: AxisValue,
    pub throttle: AxisValue,
    pub pitch: AxisValue,
    pub roll: AxisValue,
}
