mod feeder;
mod poller;
mod ui_updater;

pub use feeder::Feeder;
pub use poller::{Poller, PollingEvent};
pub use ui_updater::UIUpdater;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    NextFeeder,
    Quit,
}
