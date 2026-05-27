use crate::{
    feeder::{Feeder, backend::Backend, config::FeederConfig},
    virtual_controller::VirtualControllerState,
};

pub struct ConfiguredFeeder<'a, B: Backend> {
    backend: B,
    config: &'a FeederConfig,
}

impl<'a, B: Backend> ConfiguredFeeder<'a, B> {
    pub fn new(backend: B, config: &'a FeederConfig) -> ConfiguredFeeder<'a, B> {
        ConfiguredFeeder { backend, config }
    }
}

impl<'a, B: Backend> Feeder for ConfiguredFeeder<'a, B> {
    type Error = B::Error;

    fn feed(&mut self, state: &VirtualControllerState) -> Result<(), Self::Error> {
        for (idx, btn) in self.config.buttons.iter().enumerate() {
            self.backend.set_button(idx, state.get_button(btn))?;
        }

        for (idx, axis) in self.config.axes.iter().enumerate() {
            self.backend.set_axis(idx, state.get_axis(axis))?;
        }

        if self.config.hat {
            self.backend.set_hat(state.hat)?;
        }

        Ok(())
    }
}
