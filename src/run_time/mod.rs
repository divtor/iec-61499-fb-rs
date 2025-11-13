use crate::{
    fb_impl::voter::Fb,
    run_time::connection::{Connection, FannedConnection},
};

pub mod connection;

pub struct RunTime {
    pub connections: Vec<Connection>,
    pub fanned_connections: Vec<FannedConnection>,
    pub fbs: Vec<(usize, Box<dyn Fb>)>,
}

// maybe rethink to handle connections via lifetimes?
impl RunTime {
    fn _send_data(&mut self, id: usize) {
        if let Some((_, fb)) = self.fbs.iter().find(|(id, _)| id == id) {
            if let Some(out_event) = fb.active_out_event() {
                let associated = fb.event_associations(out_event);

                for _data in associated {}
            }
        }

        let relevant_connections = self
            .connections
            .iter()
            .filter(|c| c.from.function_block_id == id);
    }
}
