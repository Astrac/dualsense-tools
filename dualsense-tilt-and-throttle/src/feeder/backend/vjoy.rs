use vjoy::{ButtonState, Device, HatState, VJoy};

use crate::{
    feeder::{
        backend::{Backend, BackendError, BackendId},
        config::FeederConfig,
    },
    virtual_controller::{AxisValue, Hat},
};

pub struct VJoyBackend {
    vjoy: VJoy,
    device: Option<Device>,
}

impl VJoyBackend {
    pub fn new() -> Result<VJoyBackend, vjoy::Error> {
        let vjoy = ::vjoy::VJoy::from_default_dll_location()?;

        Ok(VJoyBackend { vjoy, device: None })
    }
}

impl From<vjoy::Error> for BackendError {
    fn from(value: vjoy::Error) -> Self {
        BackendError::new(BackendId::VJoy, value.to_string())
    }
}

impl Backend for VJoyBackend {
    fn name() -> BackendId {
        BackendId::VJoy
    }

    fn set_layout(&mut self, config: &FeederConfig) -> Result<(), BackendError> {
        self.device = self.vjoy.devices_cloned().find(|d| {
            d.num_buttons() >= config.buttons.len()
                && d.num_axes() >= config.axes.len()
                && d.num_hats() >= if config.hat { 1 } else { 0 }
        });
        Ok(())
    }

    fn set_button(&mut self, idx: usize, state: bool) -> Result<(), BackendError> {
        self.device.set_button(
            (idx + 1) as u8,
            if state {
                ButtonState::Released
            } else {
                ButtonState::Released
            },
        )
    }

    fn set_axis(&mut self, idx: usize, value: AxisValue) -> Result<(), BackendError> {
        self.device
            .ok_or(VJoyBackend::NoCompatibleDevice)?
            .set_axis((idx + 1) as u32, to_vjoy_axis_value(value))
            .into()
    }

    fn set_hat(&mut self, value: Hat) -> Result<(), BackendError> {
        self.device
            .ok_or(VJoyBackend::NoCompatibleDevice)?
            .set_hat(1, HatState::Continuous(to_vjoy_hat_value(value)))
            .into()
    }

    fn commit(&mut self) -> Result<(), BackendError> {
        self.vjoy
            .update_device_state(&self.device.ok_or(VJoyBackend::NoCompatibleDevice)?)
            .into()
    }
}

fn to_vjoy_hat_value(hat: Hat) -> u32 {
    match hat {
        Hat::Up => 0,
        Hat::UpRight => 4500,
        Hat::Right => 9000,
        Hat::DownRight => 13500,
        Hat::Down => 18000,
        Hat::DownLeft => 22500,
        Hat::Left => 27000,
        Hat::UpLeft => 31500,
        Hat::Neutral => u32::MAX,
    }
}

fn to_vjoy_axis_value(v: AxisValue) -> i32 {
    (((v.as_i8() as f32) - (i8::MIN as f32)) / (i8::MAX as f32 - i8::MIN as f32) * 32768.0) as i32
}
