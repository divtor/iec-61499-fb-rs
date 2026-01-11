use crate::{
    fb::{
        Fb,
        data::{
            Data,
            comm::DataBuffer,
            ty::{Bool, DataKind},
        },
        direction::{In, Out},
        event::{Event, ty::Signal},
    },
    fb_impl::event::dbg_state_print,
};

#[derive(Clone, Debug, Default)]
enum SrState {
    #[default]
    Q0,
    Set,
    Reset,
}

#[allow(non_camel_case_types)]
#[derive(Default, Debug)]
pub struct E_SR {
    instance_name: &'static str,
    ec_state: SrState,
    s: Event<In, Signal>,
    r: Event<In, Signal>,
    eo: Event<Out, Signal>,
    q: Data<Out, Bool>,
}

impl E_SR {
    pub fn new(instance_name: &'static str) -> Self {
        Self {
            instance_name,
            ..Default::default()
        }
    }
}

impl Fb for E_SR {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn instance_name(&self) -> &'static str {
        self.instance_name
    }

    fn data_kind(&self, data: &str) -> DataKind {
        match data {
            "q" => DataKind::Bool,
            _ => panic!("unknown data {data}"),
        }
    }

    fn set_event_in(&mut self, event: &str) {
        match event {
            "s" => self.s.receive(),
            "r" => self.r.receive(),
            _ => panic!("unknown event {event}"),
        }
    }

    fn active_event_in(&self) -> Option<&'static str> {
        let mut event = None;

        if self.s.read() {
            event = Some("s");
        }

        if self.r.read() {
            event = Some("r");
        }

        event
    }

    fn active_event_out(&self) -> Option<&'static str> {
        let mut event = None;

        if self.eo.read() {
            event = Some("eo");
        }

        event
    }

    fn clear_event_out(&mut self) {
        self.eo.reset();
    }

    fn with_for_event(&self, event: &str) -> Vec<&'static str> {
        match event {
            "eo" => vec!["q"],
            "s" | "r" => vec![],
            _ => panic!("unknown event {event}"),
        }
    }

    fn read_data_out(&self, data: &str) -> DataBuffer {
        match data {
            "q" => self.q.as_buf(),
            _ => panic!("unknown data {data}"),
        }
    }

    fn write_data_in(&mut self, data: &str, buf: &DataBuffer) {
        match (data, buf) {
            _ => panic!("unknown data {data} or invalid communication data variant {buf:?}"),
        }
    }

    fn invoke_execution_control(&mut self) -> bool {
        let mut unstable = false;

        match self.ec_state {
            SrState::Q0 => {
                if self.s.read_and_reset() {
                    dbg_state_print(self.instance_name, "Q0 -> SET");
                    self.enter(SrState::Set);
                    unstable = true;
                }
            }
            SrState::Set => {
                if self.r.read_and_reset() {
                    dbg_state_print(self.instance_name, "SET -> RESET");
                    self.enter(SrState::Reset);
                    unstable = true;
                }
            }
            SrState::Reset => {
                if self.s.read_and_reset() {
                    dbg_state_print(self.instance_name, "RESET -> SET");
                    self.enter(SrState::Set);
                    unstable = true;
                }
            }
        }

        unstable
    }
}

impl E_SR {
    fn enter(&mut self, state: SrState) {
        match state {
            SrState::Set => {
                self.set_algorithm();
                self.eo.send();
            }
            SrState::Reset => {
                self.reset_algorithm();
                self.eo.send();
            }
            _ => panic!("transition is not modelled"),
        }

        self.ec_state = state;
    }
}

impl E_SR {
    fn set_algorithm(&mut self) {
        self.q.write(true);
    }

    fn reset_algorithm(&mut self) {
        self.q.write(false);
    }
}

impl std::fmt::Display for E_SR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}={{s={}, r={}, eo={}, q={}, state={:?}}}",
            self.instance_name,
            self.s.read(),
            self.r.read(),
            self.eo.read(),
            self.q.as_buf(),
            self.ec_state,
        )
    }
}
