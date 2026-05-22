use crate::{Timestamped, state::DualsenseState};
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct StateEvent<'a> {
    pub previous: &'a Timestamped<DualsenseState>,
    pub current: &'a Timestamped<DualsenseState>,
    pub evicted: Timestamped<DualsenseState>,
}

impl <'a> StateEvent<'a> {
    pub fn elapsed_time(&self) -> Duration {
        self.current.timestamp.duration_since(self.previous.timestamp)
    }
}
