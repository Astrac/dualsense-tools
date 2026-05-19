use super::Feeder;
use vjoy::{ButtonState, Device, Error, VJoy};

use crate::{
    emulated::{EmulatedGamepad, EmulatedHat},
    emulated_axis_value::EmulatedAxisValue,
};

pub struct VJoy {
    vjoy: VJoy,
    device: Device,
}

impl Feeder for VJoy {
    fn feed_state(&mut self, state: &EmulatedGamepad) -> Result<(), Error> {
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
        EmulatedHat::Neutral => -1,
    }
}

fn to_vjoy_axis_value(v: EmulatedAxisValue) -> i32 {
    (v.as_i8() as i32) << 24
}
