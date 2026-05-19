use crate::emulated::{EmulatedAxes, EmulatedGamepad};
use dualsense_tools::{Dualsense, Tilt, TiltEstimator, state::DualsenseState};
use std::time::Instant;

pub struct Emulator<const N: usize> {
    device: Dualsense,
    tilt_estimator: TiltEstimator<N>,
    dualsense_state: DualsenseState,
    tilt: Tilt,
    current_timestamp: Instant,
    last_timestamp: Instant,
}

impl<const N: usize> Emulator<N> {
    pub fn new(device: Dualsense, tilt_estimator: TiltEstimator<N>) -> Emulator<N> {
        Emulator {
            device,
            tilt_estimator,
            dualsense_state: DualsenseState::default(),
            tilt: Tilt::default(),
            current_timestamp: Instant::now(),
            last_timestamp: Instant::now(),
        }
    }
}

impl<const N: usize> Iterator for Emulator<N> {
    type Item = EmulatedGamepad;

    fn next(&mut self) -> Option<Self::Item> {
        let ds_state = &mut self.dualsense_state;
        self.last_timestamp = self.current_timestamp;
        self.current_timestamp = Instant::now();

        // TODO: Handle errors
        self.device.read_into(ds_state).unwrap();

        self.tilt = self
            .tilt_estimator
            .next_estimate(
                &ds_state.accel,
                &ds_state.gyro,
                &self.current_timestamp.duration_since(self.last_timestamp),
            )
            .accel_corrected_gyro;

        let throttle: i8 =
            ((ds_state.axes.rz.as_u8() / 2) as i8) - ((ds_state.axes.z.as_u8() / 2) as i8);

        let emulated_state = EmulatedGamepad {
            axes: EmulatedAxes {
                x: ds_state.axes.x.into(),
                y: ds_state.axes.y.into(),
                rx: ds_state.axes.rx.into(),
                ry: ds_state.axes.ry.into(),
                throttle: throttle.into(),
                pitch: self.tilt.get_pitch_radians().into(),
                roll: self.tilt.get_roll_radians().into(),
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
        };

        Some(emulated_state)
    }
}
