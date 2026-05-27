use crate::{
    feeder::{backend::Backend, config::FeederConfig},
    virtual_controller::{AxisValue, Hat},
};

pub struct Disabled;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum NoError {}

impl Backend for Disabled {
    type Error = NoError;

    fn name() -> &'static str {
        "Disabled"
    }

    fn set_layout(&mut self, _config: FeederConfig) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_button(&mut self, _idx: usize, _state: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_axis(&mut self, _idx: usize, _value: AxisValue) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_hat(&mut self, _value: Hat) -> Result<(), Self::Error> {
        Ok(())
    }

    fn commit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
