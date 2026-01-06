use crate::fb::{
    Fb,
    data::{
        Data,
        comm::DataBuffer,
        ty::{Bool, DataKind, UInt},
    },
    direction::{In, Out},
    event::{Event, ty::Signal},
};

#[derive(Clone, Debug, Default)]
enum CtuState {
    #[default]
    Start,
    Cu,
    R,
}

#[allow(non_camel_case_types)]
#[derive(Default, Debug)]
pub struct E_CTU {
    instance_name: &'static str,
    ec_state: CtuState,
    cu: Event<In, Signal>,
    r: Event<In, Signal>,
    cuo: Event<Out, Signal>,
    ro: Event<Out, Signal>,
    pv: Data<In, UInt>,
    q: Data<Out, Bool>,
    cv: Data<Out, UInt>,
}

impl E_CTU {
    pub fn new(instance_name: &'static str) -> Self {
        Self {
            instance_name,
            ..Default::default()
        }
    }
}

impl Fb for E_CTU {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn instance_name(&self) -> &'static str {
        self.instance_name
    }

    fn data_kind(&self, data: &str) -> DataKind {
        match data {
            "pv" => self.pv.as_kind(),
            "q" => self.q.as_kind(),
            "cv" => self.cv.as_kind(),
            _ => panic!("unknown data {data}"),
        }
    }

    fn set_event_in(&mut self, event: &str) {
        match event {
            "cu" => self.cu.receive(),
            "r" => self.r.receive(),
            _ => panic!("unknown event {event}"),
        }
    }

    fn active_event_in(&self) -> Option<&'static str> {
        let mut event = None;

        if self.cu.read() {
            event = Some("cu");
        }

        if self.r.read() {
            event = Some("r");
        }

        event
    }

    fn active_event_out(&self) -> Option<&'static str> {
        let mut event = None;

        if self.cuo.read() {
            event = Some("cuo");
        }

        if self.ro.read() {
            event = Some("ro");
        }

        event
    }

    fn clear_event_out(&mut self) {
        self.cuo.reset();
        self.ro.reset();
    }

    fn with_for_event(&self, event: &str) -> Vec<&'static str> {
        match event {
            "cu" => vec!["pv"],
            "cuo" | "ro" => vec!["q", "cv"],
            _ => panic!("unknown event {event}"),
        }
    }

    fn read_data_out(&self, data: &str) -> DataBuffer {
        match data {
            "q" => self.q.as_buf(),
            "cv" => self.cv.as_buf(),
            _ => panic!("unknown data {data}"),
        }
    }

    fn write_data_in(&mut self, data: &str, buf: &DataBuffer) {
        match (data, buf) {
            ("pv", DataBuffer::UInt(v)) => {
                self.pv.update(*v);
            }
            _ => panic!("unknown data {data} or invalid communication data variant {buf:?}"),
        }
    }

    fn invoke_execution_control(&mut self) -> bool {
        let mut unstable = false;

        match self.ec_state {
            CtuState::Start => {
                if self.cu.read_and_reset() && self.cv.read() < self.pv.read() {
                    self.ec_state = CtuState::Cu;
                    unstable = true;
                } else if self.r.read_and_reset() {
                    self.ec_state = CtuState::R;
                    unstable = true;
                }
            }
            CtuState::Cu => {
                self.cu_algorithm();
                self.cuo.send();
                self.ec_state = CtuState::Start;
                unstable = true;
            }
            CtuState::R => {
                self.r_algorithm();
                self.ro.send();
                self.ec_state = CtuState::Start;
                unstable = true;
            }
        }

        unstable
    }
}

impl E_CTU {
    fn r_algorithm(&mut self) {
        self.cv.write(0);
        self.q.write(false);
    }

    fn cu_algorithm(&mut self) {
        self.cv.write(self.cv.read() + 1);
        self.q.write(self.cv.read() == self.pv.read());
    }
}
