use std::collections::HashSet;

use crate::{
    fb::{
        FbRef,
        data::comm::CommunicationData,
        direction::{In, Out},
    },
    run_time::connection::{Conn, port::Port},
};

pub mod connection;

#[derive(Default)]
pub struct RunTime {
    pub conns: Vec<Conn>,
    pub fb_refs: Vec<FbRef>,
}

#[allow(dead_code)]
impl RunTime {
    pub fn connect(&mut self, from: (usize, &'static str), to: (usize, &'static str)) {
        let from = Port::<Out>::new(self.fb_refs[from.0].clone(), from.1);
        let to = Port::<In>::new(self.fb_refs[to.0].clone(), to.1);
        let buf = CommunicationData::Unassigned;

        let conn = Conn { from, to, buf };

        self.conns.push(conn);
    }

    pub fn fetching_conns(&self) -> Vec<usize> {
        let mut conn_ids = HashSet::new();

        for fb_ref in self.fb_refs.iter() {
            let fb = fb_ref.borrow();
            let name = fb.instance_name();

            if let Some(e) = fb.active_out_event() {
                let fields = fb.event_associations(e);

                if fields.is_empty() {
                    continue;
                }

                for (conn_id, conn) in self.conns.iter().enumerate() {
                    let same_name = conn.from.fb_ref.borrow().instance_name() == name;
                    let relevant_field = fields.contains(&conn.from.field);

                    if same_name && relevant_field {
                        conn_ids.insert(conn_id);
                    }
                }
            }
        }

        conn_ids.into_iter().collect()
    }

    pub fn sending_conns(&self) -> Vec<usize> {
        let mut conn_ids = HashSet::new();

        for fb_ref in self.fb_refs.iter() {
            let fb = fb_ref.borrow();
            let name = fb.instance_name();

            if let Some(e) = fb.active_in_event() {
                let fields = fb.event_associations(e);

                if fields.is_empty() {
                    continue;
                }

                for (conn_id, conn) in self.conns.iter().enumerate() {
                    let same_name = conn.to.fb_ref.borrow().instance_name() == name;
                    let relevant_field = fields.contains(&conn.to.field);

                    if same_name && relevant_field {
                        conn_ids.insert(conn_id);
                    }
                }
            }
        }

        conn_ids.into_iter().collect()
    }

    pub fn fetch_all(&mut self) {
        for conn_id in self.fetching_conns() {
            self.conns[conn_id].fetch();
        }
    }

    pub fn send_all(&self) {
        for conn_id in self.sending_conns() {
            self.conns[conn_id].send();
        }
    }
}
