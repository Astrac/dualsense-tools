mod disabled;
#[cfg(target_os = "windows")]
mod vjoy;

use crate::{
    feeder::config::FeederConfig,
    virtual_controller::{AxisValue, Hat},
};

pub trait Backend {
    type Error: std::fmt::Debug;

    fn name() -> &'static str;
    fn set_layout(&mut self, config: FeederConfig) -> Result<(), Self::Error>;
    fn set_button(&mut self, idx: usize, state: bool) -> Result<(), Self::Error>;
    fn set_axis(&mut self, idx: usize, value: AxisValue) -> Result<(), Self::Error>;
    fn set_hat(&mut self, value: Hat) -> Result<(), Self::Error>;
    fn commit(&mut self) -> Result<(), Self::Error>;
}

#[cfg(target_os = "windows")]
pub fn auto() -> impl Backend {
    vjoy::VJoyBackend::new()
}

#[cfg(not(target_os = "windows"))]
pub fn auto() -> impl Backend {
    disabled::Disabled
}
