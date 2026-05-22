use std::time::Instant;

#[derive(Clone, Copy, Debug)]
pub struct Timestamped<A> {
    pub value: A,
    pub timestamp: Instant,
}

impl<A> Timestamped<A> {
    pub fn new(value: A) -> Timestamped<A> {
        Timestamped {
            value,
            timestamp: Instant::now(),
        }
    }
}
