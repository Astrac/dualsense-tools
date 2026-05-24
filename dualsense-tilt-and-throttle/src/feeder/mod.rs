use crate::emulated::EmulatedGamepad;

#[cfg(not(target_os = "windows"))]
mod unsupported;
#[cfg(target_os = "windows")]
mod vjoy;

mod error;

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum FeederId {
    VJoy,
    Unsupported,
}

pub trait EmulatedStateFeeder {
    fn id(&self) -> FeederId;
    fn feed_state(&mut self, state: &EmulatedGamepad) -> Result<(), error::Error>;
}

pub struct Feeder;

#[cfg(target_os = "windows")]
impl Feeder {
    fn vjoy() -> Result<impl EmulatedStateFeeder, error::Error> {
        let vjoy = ::vjoy::VJoy::from_default_dll_location()?;
        let device_id = vjoy.get_id_for_configuration(13, 7, 1)?;
        let device = vjoy.get_device_state(device_id)?;

        Ok(vjoy::VJoyFeeder::new(vjoy, device))
    }

    pub fn auto() -> Result<impl EmulatedStateFeeder, error::Error> {
        Feeder::vjoy()
    }
}

#[cfg(not(target_os = "windows"))]
impl Feeder {
    pub fn auto() -> Result<impl EmulatedStateFeeder, error::Error> {
        Ok(unsupported::Unsupported)
    }
}
