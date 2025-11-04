use core::fmt;

use crate::cli::{self, args::Sequence, output::VoterInformation};

use super::{
    data::{self, Data},
    data_type::Bool,
    direction::{In, Out},
    event::Event,
};

// TODO: figure out:
// - how to associate `Data` to `Event` (WITH qualifier)
// - how to fetch new values on associated `Data`

#[derive(Clone, Debug, Default)]
enum VoterState {
    #[default]
    Ready,
    Vote,
    VotedPos,
    Reset,
}

impl VoterState {
    fn as_str(&self) -> &'static str {
        match self {
            VoterState::Ready => "Ready",
            VoterState::Vote => "Vote",
            VoterState::VotedPos => "VotedPos",
            VoterState::Reset => "Reset",
        }
    }
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct Voter {
    ecc: VoterState,
    vote: Event<In>,
    reset: Event<In>,
    voted: Event<Out>,
    ready: Event<Out>,
    a: Data<In, Bool>,
    b: Data<In, Bool>,
    c: Data<In, Bool>,
    state: Data<Out, Bool>,
}

impl Voter {
    #[allow(clippy::nonminimal_bool)]
    /// the vote algorithm implemented according to the specification
    fn vote_algorithm(&mut self) {
        let a = *self.a.read();
        let b = *self.b.read();
        let c = *self.c.read();

        let vote = (a && b) || (b && c) || (a && c);

        self.state.write(vote);
    }

    /// the reset algorithm implemented according to the specification
    fn reset_algorithm(&mut self) {
        self.state.write(false);
    }

    /// Advances the current ecc state.
    /// Returns `true` if the state has changed.
    fn invoke_ecc(&mut self) -> bool {
        let mut ecc_changed = false;

        match self.ecc {
            VoterState::Ready => {
                if self.vote.read_and_reset() {
                    self.a.update();
                    self.b.update();
                    self.c.update();

                    self.ecc = VoterState::Vote;
                    ecc_changed = true;
                }
            }
            VoterState::Vote => {
                self.vote_algorithm();

                self.voted.send();

                if *self.state.read() {
                    self.ecc = VoterState::VotedPos;
                } else {
                    self.ecc = VoterState::Ready;
                }

                ecc_changed = true;
            }
            VoterState::VotedPos => {
                if self.reset.read_and_reset() {
                    self.ecc = VoterState::Reset;
                    ecc_changed = true;
                }
            }
            VoterState::Reset => {
                self.reset_algorithm();

                self.ready.send();
                self.ecc = VoterState::Ready;
                ecc_changed = true;
            }
        }

        ecc_changed
    }
}

#[allow(clippy::from_over_into)]
impl Into<VoterInformation> for &Voter {
    fn into(self) -> VoterInformation {
        VoterInformation {
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
            a: if *self.a.read() { "TRUE" } else { "FALSE" },
            b: if *self.b.read() { "TRUE" } else { "FALSE" },
            c: if *self.c.read() { "TRUE" } else { "FALSE" },
            state: if *self.state.read() { "TRUE" } else { "FALSE" },
        }
    }
}

impl fmt::Display for Voter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = cli::output::create_voter_string(self.into());
        write!(f, "{buf}")
    }
}

// Prototyping methods (not sure if these belong here)
impl Voter {
    pub fn receive_signal(&mut self, signal: &str) {
        if self.vote.read() || self.reset.read() {
            println!("there is already a different signal active");
            return;
        }

        match signal.to_lowercase().as_str() {
            "vote" => self.vote.receive(),
            "reset" => self.reset.receive(),
            _ => println!("unkown signal \"{signal}\""),
        }
    }

    pub fn toggle_input_data(&mut self, data: &str) {
        match data.to_lowercase().as_str() {
            "a" => data::toggle(&mut self.a),
            "b" => data::toggle(&mut self.b),
            "c" => data::toggle(&mut self.c),
            _ => println!("unkown input data \"{data}\""),
        }
    }

    pub fn run_voter_until_stable(&mut self) {
        let mut not_stable = true;

        while not_stable {
            not_stable = self.invoke_ecc();
        }
    }

    pub fn invoke_ecc_once(&mut self) {
        self.invoke_ecc();
    }
}

pub fn run_sequence(sequence: Sequence) {
    let mut voter = Voter::default();

    // setup voter according to sequence
    match sequence {
        Sequence::PositiveVote => {
            voter.toggle_input_data("a");
            voter.toggle_input_data("c");

            voter.receive_signal("vote");
        }
        Sequence::NegativeVote => {
            voter.toggle_input_data("a");

            voter.receive_signal("vote");
        }
        Sequence::VotedReset => {
            voter.toggle_input_data("a");
            voter.toggle_input_data("c");

            voter.receive_signal("vote");
        }
        Sequence::UnvotedReset => {
            voter.receive_signal("reset");
        }
    }

    voter.run_voter_until_stable();

    if matches!(sequence, Sequence::VotedReset) {
        voter.receive_signal("reset");
        voter.run_voter_until_stable();
    }
}
