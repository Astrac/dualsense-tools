use std::f32::consts::PI;

use dualsense_tools::{Radians, state::DualsenseAxisValue};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct EmulatedAxisValue(i8);

impl EmulatedAxisValue {
    pub fn as_i8(&self) -> i8 {
        self.0
    }
}

impl From<DualsenseAxisValue> for EmulatedAxisValue {
    fn from(val: DualsenseAxisValue) -> Self {
        let dsv: u8 = val.into();

        let emv: i8 = if dsv >= 128 {
            (dsv - 128) as i8
        } else {
            (dsv as i8) - 127 - 1
        };

        EmulatedAxisValue(emv)
    }
}

impl From<i8> for EmulatedAxisValue {
    fn from(val: i8) -> Self {
        EmulatedAxisValue(val)
    }
}

impl From<Radians> for EmulatedAxisValue {
    fn from(val: Radians) -> Self {
        let value = (val.get_angle() / PI * 2.) * (i8::MAX as f32);
        EmulatedAxisValue(value as i8)
    }
}

#[test]
fn emulated_axis_value_from_dualsense_value() {
    let ds_values: [DualsenseAxisValue; 5] = [0, 100, 128, 200, 255].map(|v| v.into());
    let expected_values: [EmulatedAxisValue; 5] = [-128, -28, 0, 72, 127].map(|v| v.into());
    let got_values: [EmulatedAxisValue; 5] = ds_values.map(|v| v.into());

    assert_eq!(expected_values, got_values)
}

#[test]
fn emulated_axis_value_from_radians() {
    let radians_values: [Radians; _] =
        [0., PI, PI / 2., PI * 3. / 2., PI * 2., -PI / 2.].map(|v| v.into());
    let expected_values: [EmulatedAxisValue; _] = [0, 127, 127, 127, 127, -127].map(|v| v.into());
    let got_values: [EmulatedAxisValue; _] = radians_values.map(|v| v.into());

    assert_eq!(expected_values, got_values)
}
