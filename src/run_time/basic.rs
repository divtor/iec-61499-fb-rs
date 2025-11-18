use std::{cell::RefCell, rc::Rc};

use crate::{
    fb::{
        Fb, data,
        direction::{In, Out},
    },
    fb_impl::voter_dynamic::Voter,
    run_time::connection::{DataConn, EventConn, port::Port},
};

#[derive(Default, Debug)]
pub struct Runtime {
    data_conns: Vec<DataConn>,
    event_conns: Vec<EventConn>,
    fb_refs: Vec<Rc<RefCell<dyn Fb>>>,
}

impl Runtime {
    /// adds any struct that implements the `Fb` trait to the pool of function blocks
    pub fn add_fb<T: Fb + 'static>(&mut self, fb: T) {
        let instance_name_present = self
            .fb_refs
            .iter()
            .any(|fb_ex| fb_ex.borrow().instance_name() == fb.instance_name());

        if instance_name_present {
            println!(
                "fb with instance name {} already exists in runtime",
                fb.instance_name()
            );
            return;
        }

        self.fb_refs.push(Rc::new(RefCell::new(fb)));
    }

    /// create a `DataConn` between `Data<Out, T>` and `Data<In, T>` fields of 2 seperate function blocks
    /// -> `T` has to be the same type for both
    pub fn connect_data(&mut self, from: (usize, &'static str), to: (usize, &'static str)) {
        if from.0 == to.0 {
            println!("cannot connect a function block with itself");
            return;
        }

        let from = Port::<Out>::new(self.fb_refs[from.0].clone(), from.1);
        let to = Port::<In>::new(self.fb_refs[to.0].clone(), to.1);
        let buf = from.fb_ref.borrow().read_data(from.field);

        assert!(
            data::comm::buffer_variant_eq(&buf, &to.fb_ref.borrow().read_data(to.field)),
            "{from:?} and {to:?} use different DataBuffer variants!"
        );

        let conn = DataConn { from, to, buf };

        self.data_conns.push(conn);
    }

    /// create an `EventConn` between `Event<Out>` and `Event<In>` fields of 2 seperate function blocks
    pub fn connect_event(&mut self, from: (usize, &'static str), to: (usize, &'static str)) {
        if from.0 == to.0 {
            println!("cannot connect a function block with itself");
            return;
        }

        let from = Port::<Out>::new(self.fb_refs[from.0].clone(), from.1);
        let to = Port::<In>::new(self.fb_refs[to.0].clone(), to.1);

        let conn = EventConn { from, to };

        self.event_conns.push(conn);
    }
}

// getters
impl Runtime {
    pub fn fbs(&self) -> &Vec<Rc<RefCell<dyn Fb>>> {
        &self.fb_refs
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
            if e_conn.from_out_active() {
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
        for fb_ref in self.fb_refs.iter() {
            fb_ref.borrow_mut().clear_out_event();
        }
    }

    /// invokes the execution control of all function block
    pub fn step(&self) {
        for fb in &self.fb_refs {
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

        for fb in &self.fb_refs {
            write!(
                f,
                "{}",
                fb.borrow().as_any().downcast_ref::<Voter>().unwrap()
            )?;
        }

        write!(f, "")
    }
}
