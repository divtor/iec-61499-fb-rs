use crate::fb::{
    data::comm::DataBuffer,
    direction::{In, Out},
};

use port::Port;

// Data -------------------------------------------------------------------------------------------

/// connects `Data` outputs with `Data` inputs
#[derive(Debug)]
pub struct DataConn {
    pub from: Port<Out>,
    pub to: Port<In>,
    pub buf: DataBuffer,
}

impl DataConn {
    pub fn load_from(&mut self) {
        self.buf = self
            .from
            .fb_ref
            .borrow()
            .read_out_data(self.from.field)
            .clone();
    }

    pub fn fetch_to(&self) {
        self.to
            .fb_ref
            .borrow_mut()
            .write_in_data(self.to.field, &self.buf);
    }
}

// utils for easier usage for now
impl DataConn {
    pub fn from_name(&self) -> &'static str {
        self.from.fb_ref.borrow().instance_name()
    }

    pub fn to_name(&self) -> &'static str {
        self.to.fb_ref.borrow().instance_name()
    }
}

impl std::fmt::Display for DataConn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let from_name = self.from_name();
        let from_field = self.from.field;
        let to_name = self.to_name();
        let to_field = self.to.field;
        let buf = &self.buf;

        write!(
            f,
            "({from_name}, {from_field})------{buf}----->({to_name}, {to_field})"
        )
    }
}

// Event ------------------------------------------------------------------------------------------

/// connects `Event` outputs with `Event` inputs
#[derive(Debug)]
pub struct EventConn {
    pub from: Port<Out>,
    pub to: Port<In>,
}

impl EventConn {
    /// sends notification of the `from` event output to the `to` event input.
    /// indicates successful sending with boolean flag
    pub fn send(&self) -> bool {
        let mut sent = false;
        let mut f = self.from.fb_ref.borrow_mut();
        let mut t = self.to.fb_ref.borrow_mut();

        if let Some(e) = f.active_out_event() {
            let relevant_field = e == self.from.field;
            let no_active_in = t.active_in_event().is_none();

            if relevant_field && no_active_in {
                t.receive_event(self.to.field);
                f.clear_out_event();
                sent = true;
            }
        }

        sent
    }
}

// utils for easier usage for now
impl EventConn {
    pub fn from_out_active(&self) -> bool {
        self.from.fb_ref.borrow().active_out_event().is_some()
    }

    pub fn to_in_active(&self) -> bool {
        self.to.fb_ref.borrow().active_in_event().is_some()
    }

    pub fn from_name(&self) -> &'static str {
        self.from.fb_ref.borrow().instance_name()
    }

    pub fn to_name(&self) -> &'static str {
        self.to.fb_ref.borrow().instance_name()
    }

    pub fn from_out_fields(&self) -> Vec<&'static str> {
        if !self.from_out_active() {
            return vec![];
        }

        let fb_from = self.from.fb_ref.borrow();
        let event = fb_from.active_out_event().unwrap();

        fb_from.event_associations(event)
    }

    pub fn to_in_fields(&self) -> Vec<&'static str> {
        if !self.to_in_active() {
            return vec![];
        }

        let fb_to = self.to.fb_ref.borrow();
        let event = fb_to.active_in_event().unwrap();

        fb_to.event_associations(event)
    }
}

impl std::fmt::Display for EventConn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let from_name = self.from_name();
        let from_field = self.from.field;
        let to_name = self.to_name();
        let to_field = self.to.field;

        write!(
            f,
            "({from_name}, {from_field})----------->({to_name}, {to_field})"
        )
    }
}

// Port -------------------------------------------------------------------------------------------
pub mod port {
    use crate::fb::{SharedFunctionBlockRefCell, direction::Direction};

    /// represents in/output as references to function blocks including the relevant field name
    #[derive(Debug)]
    pub struct Port<D: Direction> {
        pub fb_ref: SharedFunctionBlockRefCell,
        pub field: &'static str,

        _direction_marker: std::marker::PhantomData<D>,
    }

    // TODO: ensure that `Event<In>` and `Data<In, _>` is used with `Port<In>` and vice versa
    impl<D: Direction> Port<D> {
        pub fn new(fb_ref: SharedFunctionBlockRefCell, field: &'static str) -> Self {
            Port {
                fb_ref,
                field,
                _direction_marker: std::marker::PhantomData,
            }
        }
    }
}
