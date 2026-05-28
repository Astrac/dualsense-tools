use tokio::sync::broadcast::{Receiver, Sender};

use crate::threads::{Command, PollingEvent};

pub struct Channels {
    pub polling: Channel<PollingEvent>,
    pub commands: Channel<Command>,
}

impl Channels {
    pub fn new() -> Channels {
        let (polling, _) = tokio::sync::broadcast::channel(100);
        let (commands, _) = tokio::sync::broadcast::channel(100);

        Channels {
            polling: Channel(polling),
            commands: Channel(commands),
        }
    }
}

pub struct Channel<A>(Sender<A>);

impl<A> Channel<A> {
    pub fn subscribe(&self) -> Receiver<A> {
        self.0.subscribe()
    }

    pub fn dispatch(&self) -> Sender<A> {
        self.0.clone()
    }
}
