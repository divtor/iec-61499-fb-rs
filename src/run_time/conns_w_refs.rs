use std::{cell::RefCell, rc::Rc};

use crate::{
    fb::{
        Fb, data,
        direction::{In, Out},
    },
    fb_impl::voter_dynamic::Voter,
};

use conns::{DataConn, EventConn};
use port::Port;

#[derive(Default, Debug)]
pub struct Runtime {
    data_conns: Vec<DataConn>,
    event_conns: Vec<EventConn>,
    fbs: Vec<Rc<RefCell<dyn Fb>>>,
}

impl Runtime {
    /// adds any struct that implements the `Fb` trait to the pool of function blocks
    pub fn add_fb<T: Fb + 'static>(&mut self, fb: T) {
        let instance_name_present = self
            .fbs
            .iter()
            .any(|fb_ex| fb_ex.borrow().instance_name() == fb.instance_name());

        if instance_name_present {
            println!(
                "fb with instance name {} already exists in runtime",
                fb.instance_name()
            );
            return;
        }

        self.fbs.push(Rc::new(RefCell::new(fb)));
    }

    /// create a `DataConn` between `Data<Out, T>` and `Data<In, T>` fields of 2 seperate function blocks
    /// -> `T` has to be the same type for both
    pub fn connect_data(&mut self, from: (usize, &'static str), to: (usize, &'static str)) {
        if from.0 == to.0 {
            println!("cannot connect a function block with itself");
            return;
        }

        let from = Port::<Out>::new(self.fbs[from.0].clone(), from.1);
        let to = Port::<In>::new(self.fbs[to.0].clone(), to.1);
        let buf = from.fb_ref.borrow().read_out_data(from.field);

        {
            let data_kind_eq = data::ty::kind_eq(
                &from.fb_ref.borrow().data_kind(from.field),
                &to.fb_ref.borrow().data_kind(to.field),
            );

            if !data_kind_eq {
                println!("({from}) and ({to}) use different DataTypes!");
                return;
            }
        }

        self.data_conns.push(DataConn { from, to, buf });
    }

    /// create an `EventConn` between `Event<Out>` and `Event<In>` fields of 2 seperate function blocks
    pub fn connect_event(&mut self, from: (usize, &'static str), to: (usize, &'static str)) {
        if from.0 == to.0 {
            println!("cannot connect a function block with itself");
            return;
        }

        let from = Port::<Out>::new(self.fbs[from.0].clone(), from.1);
        let to = Port::<In>::new(self.fbs[to.0].clone(), to.1);

        self.event_conns.push(EventConn { from, to });
    }
}

// getters
impl Runtime {
    pub fn fbs(&self) -> &Vec<Rc<RefCell<dyn Fb>>> {
        &self.fbs
    }

    pub fn event_conns(&self) -> &Vec<EventConn> {
        &self.event_conns
    }

    pub fn data_conns(&self) -> &Vec<DataConn> {
        &self.data_conns
    }
}

impl Runtime {
    /// updates all data connection buffers where the `from` function block has an active out event
    pub fn update_buffers(&mut self) {
        for e_conn in &self.event_conns {
            if !e_conn.from_out_active() {
                continue;
            }

            let name = e_conn.from_name();
            let fields = e_conn.from_out_fields();

            if !fields.is_empty() {
                for d_conn in self
                    .data_conns
                    .iter_mut()
                    .filter(|dc| name == dc.from_name() && fields.contains(&dc.from.field))
                {
                    d_conn.load_from();
                }
            }

            if !e_conn.send() {
                println!("{name} failed to send event");
            }
        }
    }

    /// reads all data connection buffers into the associated data fields where 'to' function block has an active in event
    pub fn read_buffers(&mut self) {
        for e_conn in &self.event_conns {
            if e_conn.to_in_active() {
                let name = e_conn.to_name();
                let fields = e_conn.to_in_fields();

                if !fields.is_empty() {
                    for d_conn in self
                        .data_conns
                        .iter_mut()
                        .filter(|dc| name == dc.to_name() && fields.contains(&dc.to.field))
                    {
                        d_conn.fetch_to();
                    }
                }
            }
        }
    }

    /// clears all out events of function blocks
    pub fn clear_out_events(&self) {
        for fb_ref in self.fbs.iter() {
            fb_ref.borrow_mut().clear_out_event();
        }
    }

    /// invokes the execution control of all function block
    pub fn step(&self) {
        for fb in &self.fbs {
            fb.borrow_mut().invoke_execution_control();
        }
    }
}

impl std::fmt::Display for Runtime {
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

        for fb in &self.fbs {
            write!(
                f,
                "{}",
                fb.borrow().as_any().downcast_ref::<Voter>().unwrap()
            )?;
        }

        write!(f, "")
    }
}

pub mod conns {
    use crate::fb::{
        data::comm::DataBuffer,
        direction::{In, Out},
    };

    use super::port::Port;

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

            fb_from.with(event)
        }

        pub fn to_in_fields(&self) -> Vec<&'static str> {
            if !self.to_in_active() {
                return vec![];
            }

            let fb_to = self.to.fb_ref.borrow();
            let event = fb_to.active_in_event().unwrap();

            fb_to.with(event)
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
}

pub mod port {
    use std::{cell::RefCell, rc::Rc};

    use crate::fb::{Fb, direction::Direction};

    /// represents in/output as references to function blocks including the relevant field name
    #[derive(Debug)]
    pub struct Port<D: Direction> {
        pub fb_ref: Rc<RefCell<dyn Fb>>,
        pub field: &'static str,

        _direction_marker: std::marker::PhantomData<D>,
    }

    // TODO: ensure that `Event<In>` and `Data<In, _>` is used with `Port<In>` and vice versa
    impl<D: Direction> Port<D> {
        pub fn new(fb_ref: Rc<RefCell<dyn Fb>>, field: &'static str) -> Self {
            Port {
                fb_ref,
                field,
                _direction_marker: std::marker::PhantomData,
            }
        }
    }

    impl<D: Direction> std::fmt::Display for Port<D> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}, {}",
                self.fb_ref.borrow().instance_name(),
                self.field
            )
        }
    }
}
