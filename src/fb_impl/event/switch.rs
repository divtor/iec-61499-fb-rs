use crate::fb::{
    Fb,
    data::{
        Data,
        comm::DataBuffer,
        ty::{Bool, DataKind},
    },
    direction::{In, Out},
    event::{Event, ty::Signal},
};

#[derive(Clone, Debug, Default)]
enum SwitchState {
    #[default]
    Start,
    G0,
    G1,
}

#[allow(non_camel_case_types)]
#[derive(Default, Debug)]
pub struct E_SWITCH {
    instance_name: &'static str,
    ec_state: SwitchState,
    ei: Event<In, Signal>,
    eo0: Event<Out, Signal>,
    eo1: Event<Out, Signal>,
    g: Data<In, Bool>,
}

impl E_SWITCH {
    pub fn new(instance_name: &'static str) -> Self {
        Self {
            instance_name,
            ..Default::default()
        }
    }
}

impl Fb for E_SWITCH {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn instance_name(&self) -> &'static str {
        self.instance_name
    }

    fn data_kind(&self, data: &str) -> DataKind {
        match data {
            "g" => DataKind::Bool,
            _ => panic!("unknown data {data}"),
        }
    }

    fn set_event_in(&mut self, event: &str) {
        match event {
            "ei" => self.ei.receive(),
            _ => panic!("unknown event {event}"),
        }
    }

    fn active_event_in(&self) -> Option<&'static str> {
        let mut event = None;

        if self.ei.read() {
            event = Some("ei");
        }

        event
    }

    fn active_event_out(&self) -> Option<&'static str> {
        let mut event = None;

        if self.eo0.read() {
            event = Some("eo0");
        }

        if self.eo1.read() {
            event = Some("eo1");
        }

        event
    }

    fn clear_event_out(&mut self) {
        self.eo0.reset();
        self.eo1.reset();
    }

    fn with_for_event(&self, event: &str) -> Vec<&'static str> {
        match event {
            "ei" => vec!["g"],
            _ => panic!("unknown event {event}"),
        }
    }

    fn read_data_out(&self, data: &str) -> crate::fb::data::comm::DataBuffer {
        match data {
            _ => panic!("unknown data {data}"),
        }
    }

    fn write_data_in(&mut self, data: &str, buf: &crate::fb::data::comm::DataBuffer) {
        match (data, buf) {
            ("g", DataBuffer::Bool(v)) => {
                self.g.update(*v);
            }
            _ => panic!("unknown data {data} or invalid communication data variant {buf:?}"),
        }
    }

    fn invoke_execution_control(&mut self) -> bool {
        let mut unstable = false;

        match self.ec_state {
            SwitchState::Start => {
                if self.ei.read_and_reset() {
                    self.ec_state = match self.g.read() {
                        true => SwitchState::G1,
                        false => SwitchState::G0,
                    };
                    unstable = true;
                }
            }
            SwitchState::G0 => {
                self.eo0.send();
                self.ec_state = SwitchState::Start;
                unstable = true;
            }
            SwitchState::G1 => {
                self.eo1.send();
                self.ec_state = SwitchState::Start;
                unstable = true;
            }
        }

        unstable
    }
}
