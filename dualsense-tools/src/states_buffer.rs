use crate::{StateEvent, Timestamped, state::DualsenseState};
use circular_buffer::CircularBuffer;

#[derive(Clone, Debug)]
pub struct DualsenseStatesBuffer<const BUFSIZE: usize> {
    states: CircularBuffer<BUFSIZE, Timestamped<DualsenseState>>,
}

impl<const BUFSIZE: usize> Default for DualsenseStatesBuffer<BUFSIZE> {
    fn default() -> Self {
        let mut states = CircularBuffer::new();
        states.fill(Timestamped::new(DualsenseState::default()));
        Self { states }
    }
}

impl<const BUFSIZE: usize> DualsenseStatesBuffer<BUFSIZE> {
    pub fn push<'a>(&'a mut self, new_state: DualsenseState) -> StateEvent<'a> {
        // Unwrapping as the circular buffer is initialized as full
        // and states are only deleted when pushing new states
        let evicted = self.states.push_back(Timestamped::new(new_state)).unwrap();
        let current = self.states.back().unwrap();
        let previous = self.states.nth_back(1).unwrap();

        StateEvent {
            previous,
            current,
            evicted,
        }
    }

    pub fn iter_states(&self) -> impl Iterator<Item = &Timestamped<DualsenseState>> {
        self.states.iter()
    }
}
