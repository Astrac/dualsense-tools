/// Represents the value of an axis in a Dualsense controller
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct DualsenseAxisValue(u8);

impl DualsenseAxisValue {
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl From<DualsenseAxisValue> for u8 {
    fn from(val: DualsenseAxisValue) -> Self {
        val.as_u8()
    }
}

impl From<u8> for DualsenseAxisValue {
    fn from(value: u8) -> Self {
        DualsenseAxisValue(value)
    }
}

/// Represents the value of a sensor (accelerometer/gyroscope)
/// in a Dualsense controller
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct DualsenseSensorValue(i16);

impl From<i16> for DualsenseSensorValue {
    fn from(value: i16) -> Self {
        DualsenseSensorValue(value)
    }
}

impl From<DualsenseSensorValue> for f32 {
    fn from(val: DualsenseSensorValue) -> Self {
        val.0 as f32
    }
}
