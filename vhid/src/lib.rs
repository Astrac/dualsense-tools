use std::error::Error;

pub trait DeviceWriter {
    type State;
    fn write(&self, state: Self::State) -> Result<(), impl Error>;
}
