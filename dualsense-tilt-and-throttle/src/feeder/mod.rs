use crate::emulated::EmulatedGamepad;

#[cfg(target_os = "windows")]
mod vjoy;

#[cfg(not(target_os = "windows"))]
mod dummy;

pub enum Error {
    #[cfg(target_os = "windows")]
    VJoyError(vjoy::Error),
}

pub trait Feeder {
    fn feed_state(&mut self, state: &EmulatedGamepad) -> Result<(), Error>;
}
