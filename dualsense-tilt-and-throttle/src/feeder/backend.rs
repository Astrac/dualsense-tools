use crate::feeder::{EmulatedStateFeeder, error};
use crate::virtual_controller::VirtualControllerState;

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum FeederBackendId {
    VJoy,
    Unsupported,
}

pub struct FeederBackend;

#[cfg(target_os = "windows")]
impl FeederBackend {
    fn vjoy() -> Result<impl EmulatedStateFeeder, error::Error> {
        let vjoy = ::vjoy::VJoy::from_default_dll_location()?;
        let device_id = vjoy.get_id_for_configuration(13, 7, 1)?;
        let device = vjoy.get_device_state(device_id)?;

        Ok(vjoy::VJoyFeeder::new(vjoy, device))
    }

    pub fn auto() -> Result<impl EmulatedStateFeeder, error::Error> {
        FeederBackend::vjoy()
    }
}

#[cfg(not(target_os = "windows"))]
impl FeederBackend {
    pub fn auto() -> Result<impl EmulatedStateFeeder, error::Error> {
        Ok(Disabled)
    }
}

#[derive(Default, Debug)]
pub struct Disabled;

impl EmulatedStateFeeder for Disabled {
    fn feed_state(&mut self, _state: &VirtualControllerState) -> Result<(), super::error::Error> {
        Ok(())
    }

    fn backend(&self) -> FeederBackendId {
        FeederBackendId::Unsupported
    }
}
