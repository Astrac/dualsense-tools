use tokio::sync::broadcast::{Receiver, Sender};

use crate::threads::{Command, FeederEvent, PollingEvent};

pub struct Channels {
    pub polling_events: Channel<PollingEvent>,
    pub commands: Channel<Command>,
    pub feeder_events: Channel<FeederEvent>,
}

impl Channels {
    pub fn new() -> Channels {
        let (polling, _) = tokio::sync::broadcast::channel(100);
        let (commands, _) = tokio::sync::broadcast::channel(100);
        let (feeder, _) = tokio::sync::broadcast::channel(100);

        Channels {
            polling_events: Channel(polling),
            commands: Channel(commands),
            feeder_events: Channel(feeder)
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
