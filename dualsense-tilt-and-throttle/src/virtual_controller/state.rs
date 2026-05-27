use crate::virtual_controller::{Axes, AxisId, AxisValue, ButtonId, Buttons, Hat};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct VirtualControllerState {
    pub axes: Axes,
    pub hat: Hat,
    pub buttons: Buttons,
    pub is_tilt_enabled: bool,
}

impl VirtualControllerState {
    /// Gets a single axis value by identifier
    pub fn get_axis(&self, id: &AxisId) -> AxisValue {
        match id {
            AxisId::X => self.axes.x,
            AxisId::Y => self.axes.y,
            AxisId::Z => self.axes.z,
            AxisId::RX => self.axes.rx,
            AxisId::RY => self.axes.ry,
            AxisId::RZ => self.axes.rz,
            AxisId::Pitch => self.axes.pitch,
            AxisId::Roll => self.axes.roll,
            AxisId::Throttle => self.axes.throttle,
        }
    }

    /// Gets a single button value by identifier
    pub fn get_button(&self, id: &ButtonId) -> bool {
        match id {
            ButtonId::Cross => self.buttons.cross,
            ButtonId::Square => self.buttons.square,
            ButtonId::Circle => self.buttons.circle,
            ButtonId::Triangle => self.buttons.triangle,
            ButtonId::L1 => self.buttons.l1,
            ButtonId::R1 => self.buttons.r1,
            ButtonId::L2 => self.buttons.l2,
            ButtonId::R2 => self.buttons.r2,
            ButtonId::L3 => self.buttons.l3,
            ButtonId::R3 => self.buttons.r3,
            ButtonId::Opt => self.buttons.option,
            ButtonId::Share => self.buttons.share,
            ButtonId::Mic => self.buttons.mic,
            ButtonId::TouchClick => self.buttons.touch_click,
            ButtonId::Ps => self.buttons.ps,
        }
    }
}
