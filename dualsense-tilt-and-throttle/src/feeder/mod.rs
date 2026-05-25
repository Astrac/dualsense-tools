use crate::virtual_controller::VirtualControllerState;

mod backend;
mod config;
mod error;
mod feeders;

#[cfg(target_os = "windows")]
mod vjoy;

pub use backend::FeederBackend;
pub use backend::FeederBackendId;

pub trait EmulatedStateFeeder {
    fn backend(&self) -> FeederBackendId;
    fn feed_state(&mut self, state: &VirtualControllerState) -> Result<(), error::Error>;
}
