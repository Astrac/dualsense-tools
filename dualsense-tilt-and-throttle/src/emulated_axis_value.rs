use std::f32::consts::PI;

use dualsense_tools::{Radians, state::DualsenseAxisValue};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct EmulatedAxisValue(i8);

impl EmulatedAxisValue {
    pub fn as_i8(&self) -> i8 {
        self.0
    }
}

impl Into<EmulatedAxisValue> for DualsenseAxisValue {
    fn into(self) -> EmulatedAxisValue {
        let dsv = self.as_u8();
        let emv: i8;

        if dsv >= 128 {
            emv = (dsv - 128) as i8;
        } else {
            emv = (dsv as i8) - 127 - 1;
        }

        EmulatedAxisValue(emv)
    }
}

impl Into<EmulatedAxisValue> for i8 {
    fn into(self) -> EmulatedAxisValue {
        EmulatedAxisValue(self)
    }
}

impl Into<EmulatedAxisValue> for Radians {
    fn into(self) -> EmulatedAxisValue {
        let value = (self.get_angle() / PI * 2.) * (i8::MAX as f32);
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
    let expected_values: [EmulatedAxisValue; _] = [0, 0, -128, -128, 0, 127].map(|v| v.into());
    let got_values: [EmulatedAxisValue; _] = radians_values.map(|v| v.into());

    assert_eq!(expected_values, got_values)
}
