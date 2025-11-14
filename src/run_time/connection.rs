use crate::fb::{
    data::comm::CommunicationData,
    direction::{In, Out},
};

use port::Port;

/// uses reference counter pointers of function blocks to link
pub struct Conn {
    pub from: Port<Out>,
    pub to: Port<In>,
    pub buf: CommunicationData,
}

impl Conn {
    pub fn fetch(&mut self) {
        self.buf = self
            .from
            .fb_ref
            .borrow()
            .read_output(self.from.field)
            .clone();
    }

    pub fn send(&self) {
        self.to
            .fb_ref
            .borrow_mut()
            .write_input(self.to.field, &self.buf);
    }
}

pub mod port {
    use crate::fb::{FbRef, direction::Direction};

    pub struct Port<D: Direction> {
        pub fb_ref: FbRef,
        pub field: &'static str,

        _direction_marker: std::marker::PhantomData<D>,
    }

    impl<D: Direction> Port<D> {
        pub fn new(fb_ref: FbRef, field: &'static str) -> Self {
            Port {
                fb_ref,
                field,
                _direction_marker: std::marker::PhantomData,
            }
        }
    }
}
