use crate::emulated::EmulatedGamepad;

mod dummy;
#[cfg(target_os = "windows")]
mod vjoy;

mod error;

pub trait EmulatedStateFeeder {
    fn description(&self) -> String;
    fn feed_state(&mut self, state: &EmulatedGamepad) -> Result<(), error::Error>;
}

pub struct Feeder;

impl Feeder {
    fn dummy() -> Result<impl EmulatedStateFeeder, error::Error> {
        Ok(dummy::Dummy::default())
    }
}

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
        Feeder::dummy()
    }
}
