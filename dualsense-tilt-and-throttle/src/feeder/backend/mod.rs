mod backend_id;
mod error;
mod unsupported;
#[cfg(target_os = "windows")]
mod vjoy;

pub use backend_id::BackendId;
pub use error::BackendError;

use crate::{
    feeder::config::FeederConfig,
    virtual_controller::{AxisValue, Hat},
};

pub trait Backend {
    fn name(&self) -> BackendId;
    fn set_layout(&mut self, config: &FeederConfig) -> Result<(), BackendError>;
    fn set_button(&mut self, idx: usize, state: bool) -> Result<(), BackendError>;
    fn set_axis(&mut self, idx: usize, value: AxisValue) -> Result<(), BackendError>;
    fn set_hat(&mut self, value: Hat) -> Result<(), BackendError>;
    fn commit(&mut self) -> Result<(), BackendError>;
}

#[cfg(target_os = "windows")]
pub fn auto() -> impl Backend {
    &vjoy::VJoyBackend::new().unwrap()
}

#[cfg(not(target_os = "windows"))]
pub fn auto() -> impl Backend {
    unsupported::Unsupported
}
