use std::time::{Duration, Instant};

pub struct DeltaTTracker {
    last_timestamp: Instant,
}

impl DeltaTTracker {
    pub fn new() -> DeltaTTracker {
        DeltaTTracker {
            last_timestamp: Instant::now(),
        }
    }

    pub fn next_tick(&mut self) -> Duration {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_timestamp);
        self.last_timestamp = now;
        elapsed
    }
}
