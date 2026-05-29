use std::sync::{Arc, Mutex};

use crate::{
    feeder::{backend::{Backend, BackendError}, config::FeederConfig},
    virtual_controller::VirtualControllerState,
};

pub struct Feeder<B: Backend> {
    pub backend: Arc<Mutex<B>>,
    pub config: FeederConfig,
}

impl<B: Backend> Feeder<B> {
    pub fn new(backend: Arc<Mutex<B>>, config: FeederConfig) -> Result<Feeder<B>, BackendError> {
        backend.lock().unwrap().set_layout(&config)?;
        Ok(Feeder { backend, config })
    }

    pub fn feed(&mut self, state: &VirtualControllerState) -> Result<(), BackendError> {
        let mut backend = self.backend.lock().unwrap();

        for (idx, btn) in self.config.buttons.iter().enumerate() {
            backend.set_button(idx, state.get_button(btn))?;
        }

        for (idx, axis) in self.config.axes.iter().enumerate() {
            backend.set_axis(idx, state.get_axis(axis))?;
        }

        if self.config.hat {
            backend.set_hat(state.hat)?;
        }

        backend.commit()?;

        Ok(())
    }
}
