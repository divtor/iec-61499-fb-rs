//! Builds upon the `traited_voter` concept, but
//! more polished and designed with communication
//! between function blocks in mind.

use crate::{
    cli,
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
    fb_impl::voter_util::state::VoterState,
};

// TODO: make instance_name into a String -> allows for hotloading down the line
#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Voter {
    instance_name: &'static str,
    ecc: VoterState,
    vote: Event<In, Signal>,
    reset: Event<In, Signal>,
    voted: Event<Out, Signal>,
    ready: Event<Out, Signal>,
    a: Data<In, Bool>,
    b: Data<In, Bool>,
    c: Data<In, Bool>,
    state: Data<Out, Bool>,
}

impl Voter {
    pub fn new(instance_name: &'static str) -> Self {
        Self {
            instance_name,
            ..Default::default()
        }
    }
}

impl Voter {
    #[allow(clippy::nonminimal_bool)]
    /// the vote algorithm implemented according to the specification
    fn vote_algorithm(&mut self) {
        let a = self.a.read();
        let b = self.b.read();
        let c = self.c.read();

        let vote = (a && b) || (b && c) || (a && c);

        self.state.write(vote);
    }

    /// the reset algorithm implemented according to the specification
    fn reset_algorithm(&mut self) {
        self.state.write(false);
    }
}

impl Fb for Voter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn instance_name(&self) -> &'static str {
        self.instance_name
    }

    fn invoke_execution_control(&mut self) -> bool {
        let mut unstable = false;

        match self.ecc {
            VoterState::Ready => {
                if self.vote.read_and_reset() {
                    self.ecc = VoterState::Vote;
                    unstable = true;
                }
            }
            VoterState::Vote => {
                self.vote_algorithm();

                self.voted.send();

                if self.state.read() {
                    self.ecc = VoterState::VotedPos;
                } else {
                    self.ecc = VoterState::Ready;
                }

                unstable = true;
            }
            VoterState::VotedPos => {
                if self.reset.read_and_reset() {
                    self.ecc = VoterState::Reset;
                    unstable = true;
                }
            }
            VoterState::Reset => {
                self.reset_algorithm();

                self.ready.send();

                self.ecc = VoterState::Ready;
                unstable = true;
            }
        }

        unstable
    }

    fn receive_event(&mut self, event: &str) {
        match event {
            "vote" => self.vote.receive(),
            "reset" => self.reset.receive(),
            _ => panic!("unknown event {event}"), // return Error later
        }
    }

    fn active_in_event(&self) -> Option<&'static str> {
        let mut event = None;

        if self.vote.read() {
            event = Some("vote");
        }

        if self.reset.read() {
            event = Some("reset");
        }

        event
    }

    fn active_out_event(&self) -> Option<&'static str> {
        let mut event = None;

        if self.voted.read() {
            event = Some("voted");
        }

        if self.ready.read() {
            event = Some("ready");
        }

        event
    }

    fn clear_out_event(&mut self) {
        self.voted.reset();
        self.ready.reset();
    }

    fn event_associations(&self, event: &str) -> Vec<&'static str> {
        match event {
            "vote" | "reset" => vec!["a", "b", "c"],
            "voted" | "ready" => vec!["state"],
            _ => panic!("unknown event {event}"), // return Error later
        }
    }

    fn read_out_data(&self, data: &str) -> DataBuffer {
        match data {
            "state" => self.state.as_buf(),
            _ => panic!("unknown data {data}"),
        }
    }

    fn write_in_data(&mut self, data: &str, value: &DataBuffer) {
        match (data, value) {
            ("a", DataBuffer::Bool(v)) => {
                self.a.update(*v);
            }
            ("b", DataBuffer::Bool(v)) => {
                self.b.update(*v);
            }
            ("c", DataBuffer::Bool(v)) => {
                self.c.update(*v);
            }
            _ => panic!("unknown data {data} or invalid communication data variant {value:?}"),
        }
    }

    fn get_data_kind(&self, data: &str) -> DataKind {
        match data {
            "a" => self.a.as_kind(),
            "b" => self.b.as_kind(),
            "c" => self.c.as_kind(),
            "state" => self.state.as_kind(),
            _ => panic!("unknown data {data}"),
        }
    }
}

// -- printing ------------------------------------------------------------------------------------
#[allow(clippy::from_over_into)]
impl Into<cli::output::VoterInformation> for &Voter {
    fn into(self) -> cli::output::VoterInformation {
        cli::output::VoterInformation {
            ecc: self.ecc.as_str(),
            vote: if self.vote.read() {
                "RECEIVED"
            } else {
                "INACTIVE"
            },
            reset: if self.reset.read() {
                "RECEIVED"
            } else {
                "INACTIVE"
            },
            voted: if self.voted.read() {
                "SENT"
            } else {
                "INACTIVE"
            },
            ready: if self.ready.read() {
                "SENT"
            } else {
                "INACTIVE"
            },
            a: if self.a.read() { "TRUE" } else { "FALSE" },
            b: if self.b.read() { "TRUE" } else { "FALSE" },
            c: if self.c.read() { "TRUE" } else { "FALSE" },
            state: if self.state.read() { "TRUE" } else { "FALSE" },
        }
    }
}

impl std::fmt::Display for Voter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = cli::output::voter_str_w_name(self.into(), self.instance_name);
        write!(f, "{buf}")
    }
}
