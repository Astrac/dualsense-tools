pub mod backend;
mod config;
mod configured;
mod error;
mod feeders;

pub use configured::ConfiguredFeeder;
pub use feeders::Feeders;

use crate::virtual_controller::VirtualControllerState;

pub trait Feeder {
    type Error;

    fn feed(&mut self, state: &VirtualControllerState) -> Result<(), Self::Error>;
}
