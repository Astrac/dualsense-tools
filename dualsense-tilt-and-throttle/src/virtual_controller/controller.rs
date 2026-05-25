use dualsense_tools::{
    DualsenseStatesBuffer, Tilt, TiltEstimator, control_ids::ButtonId, state::DualsenseState,
};

use crate::virtual_controller::*;

pub struct VirtualController<const N: usize> {
    states_buffer: DualsenseStatesBuffer<N>,
    tilt_estimator: TiltEstimator<N>,
    tilt: Tilt,
    tilt_enabled: bool,
    tilt_switch_trigger: &'static [ButtonId],
}

impl<const N: usize> VirtualController<N> {
    pub fn new(tilt_estimator: TiltEstimator<N>) -> VirtualController<N> {
        VirtualController {
            states_buffer: DualsenseStatesBuffer::default(),
            tilt_estimator,
            tilt: Tilt::default(),
            tilt_enabled: true,
            tilt_switch_trigger: &[ButtonId::Mic],
        }
    }

    pub fn handle_dualsense(&mut self, ds_state: DualsenseState) -> VirtualControllerState {
        let state_event = self.states_buffer.push(ds_state);

        let throttle: i8 =
            ((ds_state.axes.rz.as_u8() / 2) as i8) - ((ds_state.axes.z.as_u8() / 2) as i8);

        let is_previous_tilt_switch_pressed = self
            .tilt_switch_trigger
            .iter()
            .all(|a| state_event.previous.value.get_button(a));

        let is_current_tilt_switch_pressed = self
            .tilt_switch_trigger
            .iter()
            .all(|a| state_event.current.value.get_button(a));

        if !is_previous_tilt_switch_pressed && is_current_tilt_switch_pressed {
            self.tilt_enabled = !self.tilt_enabled;
        }

        let pitch;
        let roll;
        if self.tilt_enabled {
            self.tilt = self
                .tilt_estimator
                .next_estimate(state_event)
                .accel_corrected_gyro;

            pitch = self.tilt.get_pitch_radians().into();
            roll = self.tilt.get_roll_radians().into();
        } else {
            pitch = AxisValue::default();
            roll = AxisValue::default();
        }

        VirtualControllerState {
            axes: Axes {
                x: ds_state.axes.x.into(),
                y: ds_state.axes.y.into(),
                z: ds_state.axes.z.into(),
                rx: ds_state.axes.rx.into(),
                ry: ds_state.axes.ry.into(),
                rz: ds_state.axes.rz.into(),
                throttle: throttle.into(),
                pitch,
                roll,
            },
            hat: ds_state.hat.into(),
            buttons: Buttons {
                square: ds_state.square,
                triangle: ds_state.triangle,
                circle: ds_state.circle,
                cross: ds_state.cross,
                l1: ds_state.l1,
                r1: ds_state.r1,
                l2: ds_state.l2,
                r2: ds_state.r2,
                l3: ds_state.l3,
                r3: ds_state.r3,
                option: ds_state.option,
                share: ds_state.share,
                touch_click: ds_state.touch_click,
                ps: ds_state.ps,
                mic: ds_state.mic,
            },
            is_tilt_enabled: self.tilt_enabled,
        }
    }
}
