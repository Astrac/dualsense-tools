use crate::virtual_controller::{Axes, Buttons, Hat};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct VirtualControllerState {
    pub axes: Axes,
    pub hat: Hat,
    pub buttons: Buttons,
    pub is_tilt_enabled: bool,
}
