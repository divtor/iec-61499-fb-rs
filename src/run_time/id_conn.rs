//! This runtime is used to evaluate `reference` vs. `id` connections.
//!
//! ### FunctionBlock IDs
//! In this implementation we defer the role of unique `id`s to the function block `instance_name`.
//! - The instance name will be enforced to be unique in the run time
//! - Currently a function block's `instance_name` is a `&'static str`
//!     - this means the name needs to be present in the source code in the form of a string literal (static lifetime)
//!     - if we want to dynamically add function blocks duriong runtime in future implementations,
//!       we need to redesign the `instance_name` implementation
//!
//! If this proves to be insufficient for more complicated tasks down the line,
//! we will evaluate external crates for arena allocation / specialized containers.
//! In the case that no fitting implementation exists, we might need to implement
//! our own solution.

use std::collections::HashMap;

use crate::{
    fb::{
        Fb,
        direction::{In, Out},
    }, fb_impl::voter::dynamic_disp::Voter, run_time::id_conn::port::Port
};
use conns::{DataConn, EventConn};

#[derive(Default, Debug)]
pub struct IdConnRuntime {
    fbs: std::collections::HashMap<&'static str, Box<dyn Fb>>,
    data_conns: Vec<DataConn>,
    event_conns: Vec<EventConn>,
}

impl IdConnRuntime {
    /// adds any struct that implements the `Fb` to the HashMap
    pub fn add_fb<T: Fb + 'static>(&mut self, fb: T) {
        if self.fb_exists(fb.instance_name()) {
            println!(
                "fb with instance name {} already exists in runtime",
                fb.instance_name()
            );

            return;
        }

        self.fbs.insert(fb.instance_name(), Box::new(fb));
    }

    pub fn remove_fb(&mut self, name: &'static str) {
        self.fbs.remove(name);

        self.data_conns
            .retain(|dc| dc.from.fb_name != name && dc.to.fb_name != name);

        self.event_conns
            .retain(|ec| ec.from.fb_name != name && ec.to.fb_name != name);
    }

    pub fn connect_data(
        &mut self,
        from: (&'static str, &'static str),
        to: (&'static str, &'static str),
    ) {
        if !self.connection_valid(from.0, to.0) {
            return;
        }

        let from = Port::<Out>::new(from.0, from.1);
        let to = Port::<In>::new(to.0, to.1);
        let buf = self
            .fbs
            .get(from.fb_name)
            .unwrap()
            .as_ref()
            .read_data_out(from.fb_field);

        self.data_conns.push(DataConn { from, to, buf });
    }

    pub fn connect_event(
        &mut self,
        from: (&'static str, &'static str),
        to: (&'static str, &'static str),
    ) {
        if !self.connection_valid(from.0, to.0) {
            return;
        }

        let from = Port::<Out>::new(from.0, from.1);
        let to = Port::<In>::new(to.0, to.1);

        self.event_conns.push(EventConn { from, to });
    }

    pub fn fbs(&self) -> &HashMap<&'static str, Box<dyn Fb>> {
        &self.fbs
    }

    pub fn event_conns(&self) -> &Vec<EventConn> {
        &self.event_conns
    }

    pub fn data_conns(&self) -> &Vec<DataConn> {
        &self.data_conns
    }
}

impl IdConnRuntime {
    fn fb_exists(&self, name: &'static str) -> bool {
        self.fbs.iter().any(|(n, _)| *n == name)
    }

    fn connection_valid(&self, from: &'static str, to: &'static str) -> bool {
        let mut valid = false;

        match (self.fb_exists(from), self.fb_exists(to)) {
            (true, true) => {
                valid = true;
            }
            (true, false) => {
                println!("[error to]: no fb with name=\"{to}\" exists");
            }
            (false, true) => {
                println!("[error from]: no fb with name=\"{from}\" exists");
            }
            (false, false) => {
                println!("[error from/to]: no fbs with names=\"{from}\"/\"{to}\" exists");
            }
        };

        valid
    }
}

// Data/Event Sending
impl IdConnRuntime {
    pub fn send_from(&mut self) {
        // check all event connections for active from events
        for ec in &self.event_conns {
            let from = self
                .fbs
                .get(ec.from.fb_name)
                .expect("from in send_from is invalid");

            let no_active_from_event = from.active_event_out().is_none();
            let wrong_from_event = from.active_event_out().unwrap_or("INVALID") != ec.from.fb_field;

            if no_active_from_event || wrong_from_event {
                continue;
            }

            // get associated WITH fields of data_out
            let from_name = ec.from.fb_name;
            let from_event = from.active_event_out().unwrap();
            let from_fields = from.with_for_event(from_event);

            // update all relevant data conn buffers
            if !from_fields.is_empty() {
                for dc in self
                    .data_conns
                    .iter_mut()
                    .filter(|conn| conn.from.fb_name == from_name)
                {
                    dc.buf = from.read_data_out(dc.from.fb_field);
                }
            }

            let to = self
                .fbs
                .get_mut(ec.to.fb_name)
                .expect("to in send_from invalid");

            let to_event_can_be_scheduled = to.active_event_in().is_none();
            let to_name = to.instance_name();

            if to_event_can_be_scheduled {
                to.set_event_in(ec.to.fb_field);
                println!("{from_name} sent event {} to {to_name}", ec.to.fb_field);
            } else {
                println!(
                    "{from_name} failed to send event to {to_name}, since {to_name} already has an event scheduled"
                );
            }
        }

        // Note: this is bad unless we have a strict manual execution order (that we have)
        self.clear_out_events();
    }

    pub fn read_in(&mut self) {
        // check all event connections for active to events
        for ec in &self.event_conns {
            let to = self
                .fbs
                .get_mut(ec.to.fb_name)
                .expect("to in send_from invalid");

            let no_active_to_event = to.active_event_in().is_none();
            let wrong_to_event = to.active_event_in().unwrap_or("INVALID") != ec.to.fb_field;

            if no_active_to_event || wrong_to_event {
                continue;
            }

            let to_name = ec.to.fb_name;
            let to_event = to.active_event_in().unwrap();
            let to_fields = to.with_for_event(to_event);

            // write correct data from data conn buffer to relevant target function blocks
            if !to_fields.is_empty() {
                for dc in self
                    .data_conns
                    .iter_mut()
                    .filter(|conn| conn.to.fb_name == to_name)
                {
                    to.write_data_in(dc.to.fb_field, &dc.buf);
                }
            }
        }
    }

    pub fn clear_out_events(&mut self) {
        for fb in self.fbs.values_mut() {
            fb.as_mut().clear_event_out();
        }
    }

    pub fn step(&mut self) {
        for fb in self.fbs.values_mut() {
            fb.as_mut().invoke_execution_control();
        }
    }
}

impl std::fmt::Display for IdConnRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Event connections:")?;

        for ec in &self.event_conns {
            writeln!(f, "{ec}")?;
        }

        writeln!(f)?;
        writeln!(f, "Data connections:")?;

        for dc in &self.data_conns {
            writeln!(f, "{dc}")?;
        }

        writeln!(f)?;
        writeln!(f, "Function blocks:")?;

        for fb in self.fbs.values() {
            write!(
                f,
                "{}",
                fb.as_any().downcast_ref::<Voter>().unwrap()
            )?;
        }

        write!(f, "")
    }
}

pub mod conns {
    use crate::{
        fb::{
            data::comm::DataBuffer,
            direction::{In, Out},
        },
        run_time::id_conn::port::Port,
    };

    #[derive(Debug)]
    pub struct DataConn {
        pub from: Port<Out>,
        pub to: Port<In>,
        pub buf: DataBuffer,
    }

    impl std::fmt::Display for DataConn {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let from_name = self.from.fb_name;
            let from_field = self.from.fb_field;
            let buf = &self.buf;
            let to_name = self.to.fb_name;
            let to_field = self.to.fb_field;

            write!(
                f,
                "({from_name}, {from_field})------{buf}----->({to_name}, {to_field})"
            )
        }
    }

    #[derive(Debug)]
    pub struct EventConn {
        pub from: Port<Out>,
        pub to: Port<In>,
    }

    impl std::fmt::Display for EventConn {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let from_name = self.from.fb_name;
            let from_field = self.from.fb_field;
            let to_name = self.to.fb_name;
            let to_field = self.to.fb_field;

            write!(
                f,
                "({from_name}, {from_field})--------------->({to_name}, {to_field})"
            )
        }
    }
}

pub mod port {
    use crate::fb::direction::Direction;

    /// represents in/output as references to function blocks including the relevant field name
    #[derive(Debug)]
    pub struct Port<D: Direction> {
        pub fb_name: &'static str,
        pub fb_field: &'static str,

        _direction_marker: std::marker::PhantomData<D>,
    }

    impl<D: Direction> Port<D> {
        pub fn new(fb_name: &'static str, field: &'static str) -> Self {
            Port {
                fb_name,
                fb_field: field,
                _direction_marker: std::marker::PhantomData,
            }
        }
    }

    impl<D: Direction> std::fmt::Display for Port<D> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}, {}", self.fb_name, self.fb_field)
        }
    }
}
