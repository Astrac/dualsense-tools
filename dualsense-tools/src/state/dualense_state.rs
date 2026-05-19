use crate::{
    control_ids::{AxisId, ButtonId}, hid_report, state::{DualsenseAxisValue, DualsenseSensorValue}
};

use super::{Accel, DualsenseAxes, Gyro, HatDirection};

/// Represents the state of a Dualsense controller as read from a HID report
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct DualsenseState {
    pub axes: DualsenseAxes<DualsenseAxisValue>,
    pub hat: HatDirection,
    pub triangle: bool,
    pub circle: bool,
    pub cross: bool,
    pub square: bool,
    pub l1: bool,
    pub l2: bool,
    pub r1: bool,
    pub r2: bool,
    pub l3: bool,
    pub r3: bool,
    pub share: bool,
    pub option: bool,
    pub ps: bool,
    pub touch_click: bool,
    pub mic: bool,
    pub gyro: Gyro<DualsenseSensorValue>,
    pub accel: Accel<DualsenseSensorValue>,
}

impl DualsenseState {
    /// Update the state with data read from a HID report
    pub fn update_from_hid_report(&mut self, report: &[u8; 64]) {
        hid_report::read_input_report(report, self);
    }

    /// Create a new state instance using data read from a HID report
    pub fn from_hid_report(report: &[u8; 64]) -> DualsenseState {
        let mut state = DualsenseState::default();
        hid_report::read_input_report(report, &mut state);
        state
    }

    /// Gets a single axis value by identifier
    pub fn get_axis(&self, id: AxisId) -> DualsenseAxisValue {
        match id {
            AxisId::LX => self.axes.x,
            AxisId::LY => self.axes.y,
            AxisId::LZ => self.axes.z,
            AxisId::RX => self.axes.rx,
            AxisId::RY => self.axes.ry,
            AxisId::RZ => self.axes.rz,
        }
    }

    /// Gets a single button value by identifier
    pub fn get_button(&self, id: ButtonId) -> bool {
        match id {
            ButtonId::Cross => self.cross,
            ButtonId::Square => self.square,
            ButtonId::Circle => self.circle,
            ButtonId::Triangle => self.triangle,
            ButtonId::L1 => self.l1,
            ButtonId::R1 => self.r1,
            ButtonId::L2 => self.l2,
            ButtonId::R2 => self.r2,
            ButtonId::L3 => self.l3,
            ButtonId::R3 => self.r3,
            ButtonId::Opt => self.option,
            ButtonId::Share => self.share,
            ButtonId::Mic => self.mic,
            ButtonId::TouchClick => self.touch_click,
        }
    }
}
