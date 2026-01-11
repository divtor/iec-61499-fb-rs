use super::direction::{Direction, In, Out};

use ty::EventType;

#[derive(Clone, Debug, Default)]
pub struct Event<D: Direction, T: ty::EventType> {
    _direction_marker: std::marker::PhantomData<D>,
    signal: T,
}

impl<D: Direction> Event<D, ty::Signal> {
    pub fn read(&self) -> bool {
        self.signal.get()
    }

    pub fn reset(&mut self) {
        self.signal.set(false);
    }
}

impl Event<In, ty::Signal> {
    pub fn read_and_reset(&mut self) -> bool {
        let curr = self.signal.get();

        self.signal.set(false);

        curr
    }

    pub fn receive(&mut self) {
        self.signal.set(true);
    }
}

impl Event<Out, ty::Signal> {
    pub fn send(&mut self) {
        self.signal.set(true);
    }
}

pub mod ty {
    pub trait EventType {
        type Inner;

        fn get(&self) -> Self::Inner;
        fn set(&mut self, value: Self::Inner);
    }

    #[derive(Clone, Debug, Default)]
    pub struct Signal {
        active: bool,
    }

    impl EventType for Signal {
        type Inner = bool;

        fn get(&self) -> Self::Inner {
            self.active
        }

        fn set(&mut self, value: Self::Inner) {
            self.active = value;
        }
    }
}
