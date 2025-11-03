use crate::cli::args::Sequence;

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

pub fn receive_signal(voter: &mut Voter, signal: &str) {
    match signal {
        "vote" => voter.vote.receive(),
        "reset" => voter.reset.receive(),
        _ => println!("unkown signal \"{signal}\""),
    }
}

pub fn toggle_input_data(voter: &mut Voter, data: &str) {
    let a = *voter.a.read();
    let b = *voter.b.read();
    let c = *voter.c.read();

    match data {
        "a" => data::set_explicit_value(&mut voter.a, !a),
        "b" => data::set_explicit_value(&mut voter.b, !b),
        "c" => data::set_explicit_value(&mut voter.c, !c),
        _ => println!("unkown input data \"{data}\""),
    }
}

pub fn run_voter_until_stable(voter: &mut Voter) {
    let mut not_stable = true;

    while not_stable {
        not_stable = voter.invoke_ecc();
    }
}

pub fn invoke_ecc_once(voter: &mut Voter) {
    voter.invoke_ecc();
}

pub fn run_sequence(sequence: Sequence) {
    let mut voter = Voter::default();

    // setup voter according to sequence
    match sequence {
        Sequence::PositiveVote => {
            data::set_explicit_value(&mut voter.a, true);
            data::set_explicit_value(&mut voter.c, true);

            receive_signal(&mut voter, "vote");
        }
        Sequence::NegativeVote => {
            data::set_explicit_value(&mut voter.a, true);

            receive_signal(&mut voter, "vote");
        }
        Sequence::VotedReset => {
            data::set_explicit_value(&mut voter.a, true);
            data::set_explicit_value(&mut voter.c, true);

            receive_signal(&mut voter, "vote");
        }
        Sequence::UnvotedReset => {
            receive_signal(&mut voter, "reset");
        }
    }

    run_voter_until_stable(&mut voter);

    if matches!(sequence, Sequence::VotedReset) {
        receive_signal(&mut voter, "reset");
        run_voter_until_stable(&mut voter);
    }
}
