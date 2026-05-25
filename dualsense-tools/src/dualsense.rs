use hidapi::{HidApi, HidDevice, HidResult};

use crate::state::DualsenseState;
use crate::{PRODUCT_ID, VENDOR_ID};

/// Interface used to interact with a Dualsense controller; encapsulates
/// a [hidapi::HidDevice] and a buffer to be re-used when reading input
/// reports from the hid device.
#[derive(Debug)]
pub struct Dualsense {
    device: HidDevice,
    input_buf: [u8; 64],
}

impl Dualsense {
    /// Creates a new Dualsense instance; will error if no device is
    /// connected to the USB port.
    pub fn new(hid_api: &mut HidApi) -> HidResult<Dualsense> {
        hid_api.reset_devices()?;
        hid_api.add_devices(VENDOR_ID, PRODUCT_ID)?;

        let device = hid_api.open(VENDOR_ID, PRODUCT_ID)?;

        Ok(Dualsense {
            device,
            input_buf: [0; 64],
        })
    }

    /// Reads an input report from the device into a provided mutable reference
    /// to a gamepad state.
    pub fn read_into(&mut self, state: &mut DualsenseState) -> HidResult<()> {
        self.device.read(&mut self.input_buf)?;
        state.update_from_hid_report(&self.input_buf);

        Ok(())
    }

    /// Reads an input report from the device and returns a gamepad state.
    pub fn read(&mut self) -> HidResult<DualsenseState> {
        let mut state = DualsenseState::default();
        self.read_into(&mut state)?;
        Ok(state)
    }
}
