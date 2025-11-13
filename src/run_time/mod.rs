use std::{cell::RefCell, rc::Rc};

use crate::{
    fb::direction::{In, Out},
    fb_impl::voter::Fb,
    run_time::connection::{ExternalConn, InternalConn, port::Port},
};

pub mod connection;

#[derive(Default)]
pub struct RunTime {
    pub id_connections: Vec<ExternalConn>,
    pub reference_connections: Vec<InternalConn>,
    pub fbs: Vec<Rc<RefCell<dyn Fb>>>,
}

#[allow(dead_code)]
impl RunTime {
    fn create_reference_connection(
        &mut self,
        from: (usize, &'static str),
        to: (usize, &'static str),
    ) {
        let conn = InternalConn {
            from: (self.fbs[from.0].clone(), from.1),
            to: (self.fbs[to.0].clone(), to.1),
        };

        self.reference_connections.push(conn);
    }

    fn create_id_connection(&mut self, from: (usize, &'static str), to: (usize, &'static str)) {
        let conn = ExternalConn {
            from: Port::<Out>::new(from.0, from.1),
            to: Port::<In>::new(to.0, to.1),
        };

        self.id_connections.push(conn);
    }

    fn send_via_reference(&self, conn_idx: usize) {
        let conn = &self.reference_connections[conn_idx];

        let (from_fb, from_field) = &conn.from;
        let (to_fb, to_field) = &conn.to;

        let binding = from_fb.borrow();
        let data = binding.read_output(from_field);
        to_fb.borrow_mut().write_input(to_field, data);
    }

    fn send_via_id(&self, conn_idx: usize) {
        let conn = &self.id_connections[conn_idx];

        let from_fb = &self.fbs[conn.from.function_block_id];
        let to_fb = &self.fbs[conn.to.function_block_id];

        let binding = from_fb.borrow();
        let data = binding.read_output(conn.from.field_name);
        to_fb.borrow_mut().write_input(conn.to.field_name, data);
    }
}
