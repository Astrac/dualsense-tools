mod feeding;
mod polling;
mod updating_ui;

pub use feeding::{Feeding, FeederEvent};
pub use polling::{Polling, PollingEvent};
pub use updating_ui::UpdatingUI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    NextFeeder,
    Quit,
}
