use crate::{
    control::{AxisId, ButtonId},
    state::{DualsenseAxisValue, DualsenseSensorValue},
};

use super::{Accel, DualsenseAxes, Gyro, HatDirection};

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
    pub fn update_from_hid_report(&mut self, report: &[u8; 64]) -> () {
        self.axes.x = report[1].into();
        self.axes.y = report[2].into();
        self.axes.rx = report[3].into();
        self.axes.ry = report[4].into();
        self.axes.z = report[5].into();
        self.axes.rz = report[6].into();
        self.hat = report[8].into();
        self.triangle = (report[8] & 0b10000000) != 0;
        self.circle = (report[8] & 0b01000000) != 0;
        self.cross = (report[8] & 0b00100000) != 0;
        self.square = (report[8] & 0b00010000) != 0;
        self.l1 = (report[9] & 0b00000001) != 0;
        self.r1 = (report[9] & 0b00000010) != 0;
        self.l2 = (report[9] & 0b00000100) != 0;
        self.r2 = (report[9] & 0b00001000) != 0;
        self.share = (report[9] & 0b00010000) != 0;
        self.option = (report[9] & 0b00100000) != 0;
        self.l3 = (report[9] & 0b01000000) != 0;
        self.r3 = (report[9] & 0b10000000) != 0;
        self.ps = (report[10] & 0b00000001) != 0;
        self.touch_click = (report[10] & 0b00000010) != 0;
        self.mic = (report[10] & 0b00000100) != 0;
        self.gyro.x = i16::from_le_bytes([report[16], report[17]]).into();
        self.gyro.y = i16::from_le_bytes([report[18], report[19]]).into();
        self.gyro.z = i16::from_le_bytes([report[20], report[21]]).into();
        self.accel.x = i16::from_le_bytes([report[22], report[23]]).into();
        self.accel.y = i16::from_le_bytes([report[24], report[25]]).into();
        self.accel.z = i16::from_le_bytes([report[26], report[27]]).into();
    }

    pub fn from_hid_report(self, report: &[u8; 64]) -> DualsenseState {
        let mut state = DualsenseState::default();
        state.update_from_hid_report(report);
        state
    }

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
