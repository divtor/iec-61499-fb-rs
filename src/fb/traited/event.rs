use super::{
    direction::{Direction, In, Out},
    event_type::{EventType, Signal},
};

#[derive(Clone, Debug, Default)]
pub struct Event<D: Direction> {
    _direction: D,
    signal: Signal,
}

impl<D: Direction> Event<D> {
    pub fn read(&self) -> bool {
        self.signal.get()
    }
}

impl Event<In> {
    pub fn read_and_reset(&mut self) -> bool {
        let curr = self.signal.get();

        self.signal.set(false);

        curr
    }

    pub fn receive(&mut self) {
        self.signal.set(true);
    }
}

impl Event<Out> {
    pub fn send(&mut self) {
        self.signal.set(true);
    }
}
