use crate::{
    Tilt,
    control::{AxisId, ButtonId},
    state::{Accel, DualsenseAxisValue, DualsenseSensorValue, Gyro, HatDirection},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    AccelUpdated(Accel<DualsenseSensorValue>),
    AxisChanged(AxisId, DualsenseAxisValue),
    ButtonPressed(ButtonId),
    ButtonReleased(ButtonId),
    GyroUpdated(Gyro<DualsenseSensorValue>),
    HatDirectionChanged(HatDirection),
    TiltEstimateUpdted(Tilt),
}
