#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct DualsenseAxisValue(u8);

impl DualsenseAxisValue {
    pub fn from_u8(value: u8) -> DualsenseAxisValue {
        DualsenseAxisValue(value)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl From<u8> for DualsenseAxisValue {
    fn from(value: u8) -> Self {
        DualsenseAxisValue::from_u8(value)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct DualsenseSensorValue(i16);

impl DualsenseSensorValue {
    pub fn from_i16(value: i16) -> DualsenseSensorValue {
        DualsenseSensorValue(value)
    }

    pub fn as_i16(&self) -> i16 {
        self.0
    }
}

impl From<i16> for DualsenseSensorValue {
    fn from(value: i16) -> Self {
        DualsenseSensorValue(value)
    }
}

impl Into<f32> for DualsenseSensorValue {
    fn into(self) -> f32 {
        self.0 as f32
    }
}
