use crate::{
    feeder::{backend::{Backend, BackendId, error::BackendError}, config::FeederConfig},
    virtual_controller::{AxisValue, Hat},
};

pub struct Unsupported;

impl Backend for Unsupported {
    fn name(&self) -> BackendId {
        BackendId::Unsupported
    }

    fn set_layout(&mut self, _config: &FeederConfig) -> Result<(), BackendError> {
        Ok(())
    }

    fn set_button(&mut self, _idx: usize, _state: bool) -> Result<(), BackendError> {
        Ok(())
    }

    fn set_axis(&mut self, _idx: usize, _value: AxisValue) -> Result<(), BackendError> {
        Ok(())
    }

    fn set_hat(&mut self, _value: Hat) -> Result<(), BackendError> {
        Ok(())
    }

    fn commit(&mut self) -> Result<(), BackendError> {
        Ok(())
    }
}
