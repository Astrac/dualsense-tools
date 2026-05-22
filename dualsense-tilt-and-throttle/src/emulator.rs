use crate::{
    emulated::{EmulatedAxes, EmulatedGamepad},
    emulated_axis_value::EmulatedAxisValue,
};
use dualsense_tools::{
    Dualsense, DualsenseStatesBuffer, Tilt, TiltEstimator, control_ids::ButtonId,
};

pub struct Emulator<const N: usize> {
    device: Dualsense,
    states_buffer: DualsenseStatesBuffer<N>,
    tilt_estimator: TiltEstimator<N>,
    tilt: Tilt,
    tilt_enabled: bool,
    tilt_switch_trigger: &'static [ButtonId],
    debounce_tilt_switch: bool,
}

impl<const N: usize> Emulator<N> {
    pub fn new(device: Dualsense, tilt_estimator: TiltEstimator<N>) -> Emulator<N> {
        Emulator {
            device,
            states_buffer: DualsenseStatesBuffer::default(),
            tilt_estimator,
            tilt: Tilt::default(),
            tilt_enabled: true,
            tilt_switch_trigger: &[ButtonId::Mic],
            debounce_tilt_switch: false,
        }
    }
}

impl<const N: usize> Iterator for Emulator<N> {
    type Item = EmulatedGamepad;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Handle errors
        let ds_state = self.device.read().unwrap();

        let state_event = self.states_buffer.push(ds_state);
        let throttle: i8 =
            ((ds_state.axes.rz.as_u8() / 2) as i8) - ((ds_state.axes.z.as_u8() / 2) as i8);

        if self
            .tilt_switch_trigger
            .iter()
            .all(|a| ds_state.get_button(*a))
        {
            if !self.debounce_tilt_switch {
                self.tilt_enabled = !self.tilt_enabled;
                self.debounce_tilt_switch = true;
            }
        } else {
            self.debounce_tilt_switch = false;
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
            pitch = EmulatedAxisValue::default();
            roll = EmulatedAxisValue::default();
        }

        let emulated_state = EmulatedGamepad {
            axes: EmulatedAxes {
                x: ds_state.axes.x.into(),
                y: ds_state.axes.y.into(),
                rx: ds_state.axes.rx.into(),
                ry: ds_state.axes.ry.into(),
                throttle: throttle.into(),
                pitch,
                roll,
            },
            hat: ds_state.hat.into(),
            buttons: [
                ds_state.square,
                ds_state.triangle,
                ds_state.circle,
                ds_state.cross,
                ds_state.l1,
                ds_state.r1,
                ds_state.l3,
                ds_state.r3,
                ds_state.option,
                ds_state.share,
                ds_state.touch_click,
                ds_state.ps,
                ds_state.mic,
            ],
            is_tilt_enabled: self.tilt_enabled,
        };

        Some(emulated_state)
    }
}
