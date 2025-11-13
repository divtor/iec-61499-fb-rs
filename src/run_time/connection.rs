use crate::fb::direction::{In, Out};

use port::Port;

#[allow(dead_code)]
pub struct Conn {
    pub from: Port<Out>,
    pub to: Port<In>,
}

impl Conn {
    pub fn new(from: (usize, &'static str), to: (usize, &'static str)) -> Self {
        Self {
            from: Port::new(from.0, from.1),
            to: Port::new(to.0, to.1),
        }
    }
}

pub struct FannedConn {
    pub from: Port<Out>,
    pub to: Vec<Port<In>>,
}

impl FannedConn {
    pub fn new(from: (usize, &'static str), to: Vec<(usize, &'static str)>) -> Self {
        let mut tos = Vec::new();

        for t in to {
            tos.push(Port::new(t.0, t.1));
        }

        Self {
            from: Port::new(from.0, from.1),
            to: tos,
        }
    }
}

pub mod port {
    use crate::fb::direction::Direction;

    pub struct Port<D: Direction> {
        pub function_block_id: usize,
        pub field_name: &'static str,

        _direction_marker: std::marker::PhantomData<D>,
    }

    impl<D: Direction> Port<D> {
        pub fn new(function_block_id: usize, field_name: &'static str) -> Self {
            Port {
                function_block_id,
                field_name,
                _direction_marker: std::marker::PhantomData,
            }
        }
    }
}
