use crate::prototype::signals::event_type::EventType;

use super::{
    direction::{Direction, In, Out},
    event_type::Signal,
};

#[derive(Clone, Debug, Default)]
pub struct Event<D: Direction> {
    _direction: D,
    signal: Signal,
}

impl<D: Direction> Event<D> {
    pub fn receive(&mut self) {
        self.signal.set(true);
    }
}

impl Event<In> {
    // TODO: this seems false
    pub fn read_and_reset(&mut self) -> bool {
        let curr = self.signal.get();

        self.signal.set(false);

        curr
    }
}

impl Event<Out> {
    // TODO: how to model sending in my context (when does it get set to "false" again)
    pub fn send(&mut self) {
        self.signal.set(true);
    }
}
