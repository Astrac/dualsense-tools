use super::EmulatedStateFeeder;
use vjoy::{ButtonState, Device, VJoy};

use crate::{
    emulated::{EmulatedGamepad, EmulatedHat},
    emulated_axis_value::EmulatedAxisValue,
};

pub struct VJoyFeeder {
    vjoy: VJoy,
    device: Device,
}

impl VJoyFeeder {
    pub fn new(vjoy: VJoy, device: Device) -> VJoyFeeder {
        VJoyFeeder { vjoy, device }
    }
}

impl EmulatedStateFeeder for VJoyFeeder {
    fn feed_state(&mut self, state: &EmulatedGamepad) -> Result<(), super::error::Error> {
        for (index, button) in state.buttons.iter().enumerate() {
            self.device.set_button(
                index as u8 + 1,
                if *button {
                    ButtonState::Pressed
                } else {
                    ButtonState::Released
                },
            )?;
        }

        let axes = state.axes;
        let axes_values = [
            axes.x,
            axes.y,
            axes.rx,
            axes.ry,
            axes.throttle,
            axes.roll,
            axes.pitch,
        ]
        .map(to_vjoy_axis_value);

        for (index, value) in axes_values.iter().enumerate() {
            self.device.set_axis(index as u32 + 1, *value)?;
        }

        self.device
            .set_hat(1, vjoy::HatState::Continuous(to_vjoy_hat_value(state.hat)))?;

        self.vjoy.update_device_state(&self.device)?;

        Ok(())
    }

    fn description(&self) -> String {
        "VJoy Feeder".to_owned()
    }
}

fn to_vjoy_hat_value(hat: EmulatedHat) -> u32 {
    match hat {
        EmulatedHat::Up => 0,
        EmulatedHat::UpRight => 4500,
        EmulatedHat::Right => 9000,
        EmulatedHat::DownRight => 13500,
        EmulatedHat::Down => 18000,
        EmulatedHat::DownLeft => 22500,
        EmulatedHat::Left => 27000,
        EmulatedHat::UpLeft => 31500,
        EmulatedHat::Neutral => u32::MAX,
    }
}

fn to_vjoy_axis_value(v: EmulatedAxisValue) -> i32 {
    (v.as_i8() as i32) << 24
}
